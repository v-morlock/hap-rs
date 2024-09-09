use tokio;

use hap::{
    accessory::{motion_sensor::MotionSensorAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::HapCharacteristic,
    serde_json::Value,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    Config, MacAddress, Pin, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut sensor = MotionSensorAccessory::new(
        1,
        AccessoryInformation {
            name: "Acme Sensor".into(),
            ..Default::default()
        },
    )?;

    let mut storage = FileStorage::current_dir().await?;

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await?;
            config
        },
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
                name: "Acme Sensor".into(),
                device_id: MacAddress::from([10, 20, 30, 40, 50, 63]),
                category: AccessoryCategory::Sensor,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(&sensor).await?;

    let handle = server.run_handle();

    let value_set_interval = async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;

        loop {
            interval.tick().await;

            sensor
                .motion_sensor
                .motion_detected
                .set_value(Value::Bool(true))
                .await?;

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            sensor
                .motion_sensor
                .motion_detected
                .set_value(Value::Bool(false))
                .await?;
        }

        #[allow(unreachable_code)]
        Ok(())
    };

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    futures::try_join!(handle, value_set_interval)?;

    Ok(())
}
