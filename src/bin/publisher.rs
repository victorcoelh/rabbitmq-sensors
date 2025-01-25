use std::env;
use std::time::Duration;

use rabbitmq_sensor::entities::sensor::generate_sensor_reading;
use rabbitmq_sensor::networking::BrokerConnection;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect();

    let sensor_name = args.get(1).expect("Please insert a sensor name.").clone();
    let sensor_type = args.get(2).expect("Missing a sensor type.").clone();
    let min_value = args
        .get(3)
        .expect("Missing minimum value.")
        .parse()
        .expect("Minimum value should be a numerical value.");
    let max_value = args
        .get(4)
        .expect("Missing maximum value.")
        .parse()
        .expect("Maximum value should be a numerical value.");

    let handle = tokio::task::spawn(async move {
        create_iot_sensor(sensor_type, sensor_name, min_value, max_value).await
    });

    tokio::join!(handle).0.unwrap();
}

async fn create_iot_sensor(sensor_type: String, sensor_name: String, min: f32, max: f32) {
    let broker = BrokerConnection::connect(false, &sensor_type, &sensor_name).await;

    broker
        .send_message(&format!("{sensor_type}.{sensor_name}: online"))
        .await;

    loop {
        let sensor_reading = generate_sensor_reading(min, max, 0.2);
        if sensor_reading < min || sensor_reading > max {
            broker
                .send_message(&format!("{sensor_type}.{sensor_name}: {sensor_reading:.2}"))
                .await;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
