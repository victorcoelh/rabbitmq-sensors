use rabbitmq_sensor::connection::BrokerConnection;

pub fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let broker = BrokerConnection::connect(&runtime);
    broker.send_message("Hello!", &runtime);
    println!("Mensagem enviada!");

    broker.close_connection(&runtime);
}
