use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use iced::Task;
use rabbitmq_sensor::{
    gui::{main_view, subscription, update, SensorData},
    networking::BrokerConnection,
};
use tokio::sync::Notify;

#[tokio::main]
pub async fn main() -> iced::Result {
    // initialize network to gui channel
    let (sender, receiver) = mpsc::channel();
    let gui_receiver = Arc::new(Mutex::new(receiver));

    tokio::spawn(async move {
        listen_to_broker(sender).await;
    });

    // initialize ui logic
    iced::application("RabbitMQ Sensors", update, main_view)
        .theme(|_| iced::Theme::CatppuccinMacchiato)
        .subscription(subscription)
        .run_with(move || (SensorData::new(gui_receiver), Task::none()))
}

async fn listen_to_broker(sender: Sender<String>) {
    let broker = BrokerConnection::connect(true, "", "").await;
    broker.listen_to_queue(sender).await;
    println!("Consumer listening to RabbitMQ!");

    let guard = Notify::new();
    guard.notified().await;
}
