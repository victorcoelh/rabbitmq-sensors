use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use iced::futures::Stream;
use iced::stream;
use iced::{futures::SinkExt, Subscription};
use inflector::Inflector;

use super::{Message, SensorData};

pub fn update(sensor_data: &mut SensorData, message: Message) {
    match message {
        Message::AddSensor(name) => sensor_data.add_sensor(name),
        Message::CloseDialogue(sensor) => sensor_data.errors[sensor] = None,
        Message::SensorError(name, error) => {
            if let Some(index) = sensor_data.get_index_from_name(&name) {
                sensor_data.errors[index] = Some(error);
            } else {
                sensor_data.add_sensor(name);
                let last_idx = sensor_data.errors.len() - 1;
                sensor_data.errors[last_idx] = Some(error);
            }
        }
    }
}

pub fn subscription(sensor_data: &SensorData) -> Subscription<Message> {
    Subscription::run_with_id(
        "RabbitMQ Subscription",
        get_messages_from_channel(sensor_data.receiver.clone()),
    )
}

fn get_messages_from_channel(
    receiver: Arc<Mutex<Receiver<String>>>,
) -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        loop {
            let receiver_clone = receiver.clone();
            let (name, data) = tokio::task::spawn_blocking(move || {
                parse_message(&receiver_clone.lock().unwrap().recv().unwrap())
            })
            .await
            .unwrap();

            if data == "online" {
                output.send(Message::AddSensor(name)).await.unwrap();
            } else {
                output.send(Message::SensorError(name, data)).await.unwrap();
            }
        }
    })
}

fn parse_message(received_msg: &str) -> (String, String) {
    let sensor_name = received_msg.chars().take_while(|char| *char != ':');
    let data = received_msg.chars().skip_while(|char| *char != ':').skip(2);

    let sensor_name: String = String::from_iter(sensor_name).to_title_case();
    let name: Vec<&str> = sensor_name.split(".").collect();
    let name = name.join(" ");

    (name.to_string(), String::from_iter(data))
}
