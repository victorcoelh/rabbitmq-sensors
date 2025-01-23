use std::sync::mpsc::Sender;

use amqprs::BasicProperties;
use amqprs::callbacks::DefaultChannelCallback;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, Channel, ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments};

use crate::networking::MessageConsumer;

static TOPIC_NAME: &'static str = "topic_sensors";

pub struct BrokerConnection {
    connection: Connection,
    channel: Channel,
    queue_name: String
}

impl BrokerConnection {
    pub async fn connect(is_server: bool, sensor_type: &str, sensor_name: &str) -> Self {
        let (connection, channel, queue_name) = match is_server {
            true => BrokerConnection::create_queue().await,
            false => BrokerConnection::create_exchange(sensor_type, sensor_name).await
        };

        BrokerConnection {
            connection,
            channel,
            queue_name
        }
    }

    pub async fn send_message(&self, message: &str) {
        let msg_content = message.as_bytes().to_vec();
        let publish_args = BasicPublishArguments::new(
            TOPIC_NAME,
            &self.queue_name
        );

        self.channel.basic_publish(BasicProperties::default(), msg_content, publish_args)
            .await
            .unwrap();
    }

    pub async fn listen_to_queue(&self, sender: Sender<String>) {
        let args = BasicConsumeArguments::new(
                &self.queue_name, "consumer_1"
            )
            .manual_ack(false)
            .finish();

        self.channel.basic_consume_blocking(MessageConsumer::new(sender), args)
            .await
            .unwrap();
    }

    pub async fn close_connection(self) {
        self.channel.close().await.unwrap();
        self.connection.close().await.unwrap();
    }

    async fn create_exchange(sensor_type: &str, sensor_name: &str) -> (Connection, Channel, String) {
        let (connection, channel) = BrokerConnection::get_channel().await;

        channel.exchange_declare(ExchangeDeclareArguments::new(
            TOPIC_NAME,
            "topic")
        ).await.unwrap();

        let routing_key = format!("sensors.{sensor_type}.{sensor_name}");
        (connection, channel, routing_key)
    }

    async fn create_queue() -> (Connection, Channel, String) {
        let (connection, channel) = BrokerConnection::get_channel().await;

        let (queue_name, _, _) = channel
            .queue_declare(QueueDeclareArguments::exclusive_server_named())
            .await
            .unwrap()
            .unwrap();

        channel.queue_bind(QueueBindArguments::new(
            &queue_name,
            TOPIC_NAME,
            "sensors.*.*")
        ).await.unwrap();

        (connection, channel, queue_name)
    }

    async fn get_channel() -> (Connection, Channel) {
        let connection = Connection::open(&OpenConnectionArguments::default())
        .await
        .unwrap();

        let channel = connection.open_channel(None)
            .await
            .unwrap();
        channel.register_callback(DefaultChannelCallback).await.unwrap();

        (connection, channel)
    }
}
