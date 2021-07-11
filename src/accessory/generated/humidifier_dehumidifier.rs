// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, humidifier_dehumidifier::HumidifierDehumidifierService},
	HapType,
	Result,
};

/// Humidifier-Dehumidifier accessory.
#[derive(Debug, Default)]
pub struct HumidifierDehumidifierAccessory {
    /// ID of the Humidifier-Dehumidifier Accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Humidifier-Dehumidifier service.
    pub humidifier_dehumidifier: HumidifierDehumidifierService,
}

impl HumidifierDehumidifierAccessory {
    /// Creates a new Humidifier-Dehumidifier accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let humidifier_dehumidifier_id = accessory_information.get_characteristics().len() as u64;
        let mut humidifier_dehumidifier = HumidifierDehumidifierService::new(1 + humidifier_dehumidifier_id + 1, id);
        humidifier_dehumidifier.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            humidifier_dehumidifier,
        })
    }
}

impl HapAccessory for HumidifierDehumidifierAccessory {
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
            &self.humidifier_dehumidifier,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.humidifier_dehumidifier,
        ]
    }
}

impl Serialize for HumidifierDehumidifierAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
