// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		mute::MuteCharacteristic,
		active::ActiveCharacteristic,
		volume::VolumeCharacteristic,
		volume_control_type::VolumeControlTypeCharacteristic,
		volume_selector::VolumeSelectorCharacteristic,
	},
    HapType,
};

/// Speaker service.
#[derive(Debug, Default)]
pub struct SpeakerService {
    /// Instance ID of the Speaker service.
    id: u64,
    /// [`HapType`](HapType) of the Speaker service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Mute characteristic (required).
	pub mute: MuteCharacteristic,

	/// Active characteristic (optional).
	pub active: Option<ActiveCharacteristic>,
	/// Volume characteristic (optional).
	pub volume: Option<VolumeCharacteristic>,
	/// Volume Control Type characteristic (optional).
	pub volume_control_type: Option<VolumeControlTypeCharacteristic>,
	/// Volume Selector characteristic (optional).
	pub volume_selector: Option<VolumeSelectorCharacteristic>,
}

impl SpeakerService {
    /// Creates a new Speaker service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Speaker,
			mute: MuteCharacteristic::new(id + 1 + 0, accessory_id),
			active: Some(ActiveCharacteristic::new(id + 1 + 0 + 1, accessory_id)),
			volume: Some(VolumeCharacteristic::new(id + 1 + 1 + 1, accessory_id)),
			volume_control_type: Some(VolumeControlTypeCharacteristic::new(id + 1 + 2 + 1, accessory_id)),
			volume_selector: Some(VolumeSelectorCharacteristic::new(id + 1 + 3 + 1, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for SpeakerService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.mute,
		];
		if let Some(c) = &self.active {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume_control_type {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume_selector {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.mute,
		];
		if let Some(c) = &mut self.active {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume_control_type {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume_selector {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for SpeakerService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
