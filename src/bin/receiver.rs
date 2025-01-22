use rabbitmq_sensor::networking::BrokerConnection;
use tokio::sync::Notify;

#[tokio::main]
pub async fn main() {
    let broker = BrokerConnection::connect(true, "", "").await;
    broker.listen_to_queue().await;
    println!("Consumidor iniciado!");

    let guard = Notify::new();
    guard.notified().await;
}
