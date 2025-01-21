use rabbitmq_sensor::connection::BrokerConnection;
use tokio::sync::Notify;

pub fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let broker = BrokerConnection::connect(&runtime);
    broker.listen_to_queue(&runtime);
    
    println!("Consumidor iniciado!");

    keep_alive(runtime);
}

fn keep_alive(runtime: tokio::runtime::Runtime) {
    runtime.block_on(async {
        let guard = Notify::new();
        guard.notified().await;
    });
}
