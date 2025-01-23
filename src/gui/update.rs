use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use iced::{futures::SinkExt, Subscription};
use iced::futures::Stream;
use iced::stream;

use super::{SensorData, Message};

pub fn update(sensor_data: &mut SensorData, message: Message) {
    match message {
        Message::AddSensor(name) => sensor_data.add_sensor(name),
        Message::CloseDialogue(sensor) => sensor_data.errors[sensor] = None,
    }
}

pub fn subscription(sensor_data: &SensorData) -> Subscription<Message> {
    Subscription::run_with_id("ass", get_messages_from_channel(sensor_data.receiver.clone()))
}

fn get_messages_from_channel(receiver: Arc<Mutex<Receiver<String>>>) -> impl Stream<Item = Message> {
    stream::channel(50, |mut output| async move {
        let received_msg = receiver.lock().unwrap().recv().unwrap();
        output.send(Message::AddSensor(received_msg)).await.unwrap();
    })
}
