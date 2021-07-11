// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		active::ActiveCharacteristic,
		current_humidifier_dehumidifier_state::CurrentHumidifierDehumidifierStateCharacteristic,
		target_humidifier_dehumidifier_state::TargetHumidifierDehumidifierStateCharacteristic,
		current_relative_humidity::CurrentRelativeHumidityCharacteristic,
		lock_physical_controls::LockPhysicalControlsCharacteristic,
		name::NameCharacteristic,
		relative_humidity_dehumidifier_threshold::RelativeHumidityDehumidifierThresholdCharacteristic,
		relative_humidity_humidifier_threshold::RelativeHumidityHumidifierThresholdCharacteristic,
		rotation_speed::RotationSpeedCharacteristic,
		swing_mode::SwingModeCharacteristic,
		current_water_level::CurrentWaterLevelCharacteristic,
	},
    HapType,
};

/// Humidifier-Dehumidifier service.
#[derive(Debug, Default)]
pub struct HumidifierDehumidifierService {
    /// Instance ID of the Humidifier-Dehumidifier service.
    id: u64,
    /// [`HapType`](HapType) of the Humidifier-Dehumidifier service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Active characteristic (required).
	pub active: ActiveCharacteristic,
	/// Current Humidifier-Dehumidifier State characteristic (required).
	pub current_humidifier_dehumidifier_state: CurrentHumidifierDehumidifierStateCharacteristic,
	/// Target Humidifier-Dehumidifier State characteristic (required).
	pub target_humidifier_dehumidifier_state: TargetHumidifierDehumidifierStateCharacteristic,
	/// Current Relative Humidity characteristic (required).
	pub current_relative_humidity: CurrentRelativeHumidityCharacteristic,

	/// Lock Physical Controls characteristic (optional).
	pub lock_physical_controls: Option<LockPhysicalControlsCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Relative Humidity Dehumidifier Threshold characteristic (optional).
	pub relative_humidity_dehumidifier_threshold: Option<RelativeHumidityDehumidifierThresholdCharacteristic>,
	/// Relative Humidity Humidifier Threshold characteristic (optional).
	pub relative_humidity_humidifier_threshold: Option<RelativeHumidityHumidifierThresholdCharacteristic>,
	/// Rotation Speed characteristic (optional).
	pub rotation_speed: Option<RotationSpeedCharacteristic>,
	/// Swing Mode characteristic (optional).
	pub swing_mode: Option<SwingModeCharacteristic>,
	/// Current Water Level characteristic (optional).
	pub current_water_level: Option<CurrentWaterLevelCharacteristic>,
}

impl HumidifierDehumidifierService {
    /// Creates a new Humidifier-Dehumidifier service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::HumidifierDehumidifier,
			active: ActiveCharacteristic::new(id + 1 + 0, accessory_id),
			current_humidifier_dehumidifier_state: CurrentHumidifierDehumidifierStateCharacteristic::new(id + 1 + 1, accessory_id),
			target_humidifier_dehumidifier_state: TargetHumidifierDehumidifierStateCharacteristic::new(id + 1 + 2, accessory_id),
			current_relative_humidity: CurrentRelativeHumidityCharacteristic::new(id + 1 + 3, accessory_id),
			lock_physical_controls: Some(LockPhysicalControlsCharacteristic::new(id + 1 + 0 + 4, accessory_id)),
			name: Some(NameCharacteristic::new(id + 1 + 1 + 4, accessory_id)),
			relative_humidity_dehumidifier_threshold: Some(RelativeHumidityDehumidifierThresholdCharacteristic::new(id + 1 + 2 + 4, accessory_id)),
			relative_humidity_humidifier_threshold: Some(RelativeHumidityHumidifierThresholdCharacteristic::new(id + 1 + 3 + 4, accessory_id)),
			rotation_speed: Some(RotationSpeedCharacteristic::new(id + 1 + 4 + 4, accessory_id)),
			swing_mode: Some(SwingModeCharacteristic::new(id + 1 + 5 + 4, accessory_id)),
			current_water_level: Some(CurrentWaterLevelCharacteristic::new(id + 1 + 6 + 4, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for HumidifierDehumidifierService {
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
			&self.current_humidifier_dehumidifier_state,
			&self.target_humidifier_dehumidifier_state,
			&self.current_relative_humidity,
		];
		if let Some(c) = &self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.relative_humidity_dehumidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.relative_humidity_humidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_water_level {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.current_humidifier_dehumidifier_state,
			&mut self.target_humidifier_dehumidifier_state,
			&mut self.current_relative_humidity,
		];
		if let Some(c) = &mut self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.relative_humidity_dehumidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.relative_humidity_humidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_water_level {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for HumidifierDehumidifierService {
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
