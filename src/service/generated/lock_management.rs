// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		lock_control_point::LockControlPointCharacteristic,
		version::VersionCharacteristic,
		administrator_only_access::AdministratorOnlyAccessCharacteristic,
		audio_feedback::AudioFeedbackCharacteristic,
		current_door_state::CurrentDoorStateCharacteristic,
		lock_management_auto_security_timeout::LockManagementAutoSecurityTimeoutCharacteristic,
		lock_last_known_action::LockLastKnownActionCharacteristic,
		logs::LogsCharacteristic,
		motion_detected::MotionDetectedCharacteristic,
	},
    HapType,
};

/// Lock Management service.
#[derive(Debug, Default)]
pub struct LockManagementService {
    /// Instance ID of the Lock Management service.
    id: u64,
    /// [`HapType`](HapType) of the Lock Management service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Lock Control Point characteristic (required).
	pub lock_control_point: LockControlPointCharacteristic,
	/// Version characteristic (required).
	pub version: VersionCharacteristic,

	/// Administrator Only Access characteristic (optional).
	pub administrator_only_access: Option<AdministratorOnlyAccessCharacteristic>,
	/// Audio Feedback characteristic (optional).
	pub audio_feedback: Option<AudioFeedbackCharacteristic>,
	/// Current Door State characteristic (optional).
	pub current_door_state: Option<CurrentDoorStateCharacteristic>,
	/// Lock Management Auto Security Timeout characteristic (optional).
	pub lock_management_auto_security_timeout: Option<LockManagementAutoSecurityTimeoutCharacteristic>,
	/// Lock Last Known Action characteristic (optional).
	pub lock_last_known_action: Option<LockLastKnownActionCharacteristic>,
	/// Logs characteristic (optional).
	pub logs: Option<LogsCharacteristic>,
	/// Motion Detected characteristic (optional).
	pub motion_detected: Option<MotionDetectedCharacteristic>,
}

impl LockManagementService {
    /// Creates a new Lock Management service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::LockManagement,
			lock_control_point: LockControlPointCharacteristic::new(id + 1 + 0, accessory_id),
			version: VersionCharacteristic::new(id + 1 + 1, accessory_id),
			administrator_only_access: Some(AdministratorOnlyAccessCharacteristic::new(id + 1 + 0 + 2, accessory_id)),
			audio_feedback: Some(AudioFeedbackCharacteristic::new(id + 1 + 1 + 2, accessory_id)),
			current_door_state: Some(CurrentDoorStateCharacteristic::new(id + 1 + 2 + 2, accessory_id)),
			lock_management_auto_security_timeout: Some(LockManagementAutoSecurityTimeoutCharacteristic::new(id + 1 + 3 + 2, accessory_id)),
			lock_last_known_action: Some(LockLastKnownActionCharacteristic::new(id + 1 + 4 + 2, accessory_id)),
			logs: Some(LogsCharacteristic::new(id + 1 + 5 + 2, accessory_id)),
			motion_detected: Some(MotionDetectedCharacteristic::new(id + 1 + 6 + 2, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for LockManagementService {
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
			&self.lock_control_point,
			&self.version,
		];
		if let Some(c) = &self.administrator_only_access {
		    characteristics.push(c);
		}
		if let Some(c) = &self.audio_feedback {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_door_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.lock_management_auto_security_timeout {
		    characteristics.push(c);
		}
		if let Some(c) = &self.lock_last_known_action {
		    characteristics.push(c);
		}
		if let Some(c) = &self.logs {
		    characteristics.push(c);
		}
		if let Some(c) = &self.motion_detected {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.lock_control_point,
			&mut self.version,
		];
		if let Some(c) = &mut self.administrator_only_access {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.audio_feedback {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_door_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.lock_management_auto_security_timeout {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.lock_last_known_action {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.logs {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.motion_detected {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for LockManagementService {
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
