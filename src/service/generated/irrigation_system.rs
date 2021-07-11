// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		active::ActiveCharacteristic,
		program_mode::ProgramModeCharacteristic,
		in_use::InUseCharacteristic,
		remaining_duration::RemainingDurationCharacteristic,
		name::NameCharacteristic,
		status_fault::StatusFaultCharacteristic,
	},
    HapType,
};

/// Irrigation-System service.
#[derive(Debug, Default)]
pub struct IrrigationSystemService {
    /// Instance ID of the Irrigation-System service.
    id: u64,
    /// [`HapType`](HapType) of the Irrigation-System service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Active characteristic (required).
	pub active: ActiveCharacteristic,
	/// Program Mode characteristic (required).
	pub program_mode: ProgramModeCharacteristic,
	/// In Use characteristic (required).
	pub in_use: InUseCharacteristic,

	/// Remaining Duration characteristic (optional).
	pub remaining_duration: Option<RemainingDurationCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Status Fault characteristic (optional).
	pub status_fault: Option<StatusFaultCharacteristic>,
}

impl IrrigationSystemService {
    /// Creates a new Irrigation-System service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::IrrigationSystem,
			active: ActiveCharacteristic::new(id + 1 + 0, accessory_id),
			program_mode: ProgramModeCharacteristic::new(id + 1 + 1, accessory_id),
			in_use: InUseCharacteristic::new(id + 1 + 2, accessory_id),
			remaining_duration: Some(RemainingDurationCharacteristic::new(id + 1 + 0 + 3, accessory_id)),
			name: Some(NameCharacteristic::new(id + 1 + 1 + 3, accessory_id)),
			status_fault: Some(StatusFaultCharacteristic::new(id + 1 + 2 + 3, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for IrrigationSystemService {
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
			&self.active,
			&self.program_mode,
			&self.in_use,
		];
		if let Some(c) = &self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.program_mode,
			&mut self.in_use,
		];
		if let Some(c) = &mut self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for IrrigationSystemService {
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
