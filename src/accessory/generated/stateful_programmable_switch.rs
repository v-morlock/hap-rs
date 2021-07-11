// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, stateful_programmable_switch::StatefulProgrammableSwitchService},
	HapType,
	Result,
};

/// Stateful Programmable Switch accessory.
#[derive(Debug, Default)]
pub struct StatefulProgrammableSwitchAccessory {
    /// ID of the Stateful Programmable Switch Accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Stateful Programmable Switch service.
    pub stateful_programmable_switch: StatefulProgrammableSwitchService,
}

impl StatefulProgrammableSwitchAccessory {
    /// Creates a new Stateful Programmable Switch accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let stateful_programmable_switch_id = accessory_information.get_characteristics().len() as u64;
        let mut stateful_programmable_switch = StatefulProgrammableSwitchService::new(1 + stateful_programmable_switch_id + 1, id);
        stateful_programmable_switch.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            stateful_programmable_switch,
        })
    }
}

impl HapAccessory for StatefulProgrammableSwitchAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.stateful_programmable_switch,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.stateful_programmable_switch,
        ]
    }
}

impl Serialize for StatefulProgrammableSwitchAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
