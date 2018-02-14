use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::str;
use hyper;
use hyper::server::Response;
use hyper::Uri;
use futures::{future, Future};
use rand::{self, Rng};
use crypto::{curve25519, ed25519};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use uuid::Uuid;
use futures::sync::oneshot;

use transport::http::{response, ContentType};
use transport::http::handlers::Handler;
use transport::tlv;
use db::storage::Storage;
use db::database::Database;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {
    b_pub: [u8; 32],
    a_pub: Vec<u8>,
    shared_secret: [u8; 32],
    session_key: [u8; 32],
}

pub struct PairVerify {
    session: Option<Session>,
    secret_sender: Option<oneshot::Sender<[u8; 32]>>,
}

impl PairVerify {
    pub fn new(secret_sender: oneshot::Sender<[u8; 32]>) -> PairVerify {
        PairVerify { session: None, secret_sender: Some(secret_sender) }
    }
}

impl<S: Storage> Handler<S> for PairVerify {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        if let Some(v) = decoded.get(&0x06) {
            match v[0] {
                1 => {
                    println!("/pair-verify - M1: Got Verify Start Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    if let Some(a_pub) = decoded.get(&0x03) {
                        let mut rng = rand::thread_rng();
                        let b = rng.gen::<[u8; 32]>();
                        let b_pub = curve25519::curve25519_base(&b);
                        let shared_secret = curve25519::curve25519(&b, &a_pub);

                        let accessory = Device::load::<S>(database).unwrap();
                        let mut accessory_info: Vec<u8> = Vec::new();
                        accessory_info.extend(&b_pub);
                        accessory_info.extend(accessory.id.as_bytes());
                        accessory_info.extend(a_pub);
                        let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                        let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                        let (t, v) = tlv::Type::Identifier(accessory.id).as_type_value();
                        sub_tlv.insert(t, v);
                        let (t, v) = tlv::Type::Signature(accessory_signature.to_vec()).as_type_value();
                        sub_tlv.insert(t, v);
                        let encoded_sub_tlv = tlv::encode(sub_tlv);

                        let mut session_key = [0; 32];
                        let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
                        hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Verify-Encrypt-Info", &mut session_key);

                        self.session = Some(Session {
                            b_pub,
                            a_pub: a_pub.clone(),
                            shared_secret,
                            session_key,
                        });

                        let mut encrypted_data = Vec::new();
                        let mut nonce = vec![0, 0, 0, 0];
                        nonce.extend(b"PV-Msg02");
                        let auth_tag = chacha20_poly1305_aead::encrypt(&session_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();
                        encrypted_data.extend(&auth_tag);

                        let (t, v) = tlv::Type::PublicKey(b_pub.to_vec()).as_type_value();
                        answer.insert(t, v);
                        let (t, v) = tlv::Type::EncryptedData(encrypted_data).as_type_value();
                        answer.insert(t, v);

                        println!("/pair-verify - M2: Sending Verify Start Response");
                    }
                },
                3 => {
                    println!("/pair-verify - M3: Got Verify Finish Request");

                    let (t, v) = tlv::Type::State(4).as_type_value();
                    answer.insert(t, v);

                    if let Some(ref mut session) = self.session {
                        let data = decoded.get(&0x05).unwrap();
                        let encrypted_data = Vec::from(&data[..data.len() - 16]);
                        let auth_tag = Vec::from(&data[data.len() - 16..]);

                        let mut decrypted_data = Vec::new();
                        let mut nonce = vec![0, 0, 0, 0];
                        nonce.extend(b"PV-Msg03");
                        chacha20_poly1305_aead::decrypt(&session.session_key, &nonce, &[], &encrypted_data, &auth_tag, &mut decrypted_data).unwrap();

                        let sub_tlv = tlv::decode(decrypted_data);
                        let device_pairing_id = sub_tlv.get(&0x01).unwrap();
                        let device_signature = sub_tlv.get(&0x0A).unwrap();

                        let uuid_str = str::from_utf8(device_pairing_id).unwrap();
                        let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                        let pairing = Pairing::load::<S>(pairing_uuid, database).unwrap();

                        let mut device_info: Vec<u8> = Vec::new();
                        device_info.extend(&session.a_pub);
                        device_info.extend(device_pairing_id);
                        device_info.extend(&session.b_pub);
                        if !ed25519::verify(&device_info, &pairing.public_key, &device_signature) {
                            let (t, v) = tlv::Type::Error(tlv::ErrorKind::Authentication).as_type_value();
                            answer.insert(t, v);
                        }

                        if let Some(sender) = self.secret_sender.take() {
                            sender.send(session.shared_secret).unwrap();
                        }

                        println!("/pair-verify - M4: Sending Verify Finish Response");
                    }
                },
                _ => {
                    println!("/pair-verify - M{}: Got invalid state", v[0]);
                    let (t, v) = tlv::Type::State(0).as_type_value();
                    answer.insert(t, v);
                    // TODO - return a kTLVError?
                },
            }

        }

        Box::new(future::ok(response(answer, ContentType::PairingTLV8)))
    }
}
