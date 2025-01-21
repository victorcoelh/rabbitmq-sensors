use amqprs::BasicProperties;
use amqprs::callbacks::DefaultChannelCallback;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments};

use crate::callback::MessageConsumer;

pub struct BrokerConnection {
    connection: Connection,
    channel: Channel,
    queue_name: String
}

impl BrokerConnection {
    pub fn connect(runtime: &tokio::runtime::Runtime) -> Self {
        let (connection, channel, queue_name) = runtime
            .block_on(BrokerConnection::create_channel());

        BrokerConnection {
            connection,
            channel,
            queue_name
        }
    }

    pub fn send_message(&self, message: &str, runtime: &tokio::runtime::Runtime) {
        let msg_content = message.as_bytes().to_vec();
        let channel = self.channel.clone();
        let queue_name = self.queue_name.clone();

        runtime.spawn(async move {
            let publish_args = BasicPublishArguments::new(
                "",
                &queue_name
            );
        
            channel.basic_publish(BasicProperties::default(), msg_content, publish_args)
                .await
                .unwrap();
        });
    }

    pub fn listen_to_queue(&self, runtime: &tokio::runtime::Runtime) {
        let channel = self.channel.clone();
        let queue_name = self.queue_name.clone();

        runtime.spawn(async move {
            let args = BasicConsumeArguments::new(&queue_name, "example_basic_pub_sub")
                .manual_ack(false)
                .finish();

            channel.basic_consume_blocking(MessageConsumer::default(), args)
                .await
                .unwrap();
        });
    }

    pub fn close_connection(self, runtime: &tokio::runtime::Runtime) {
        runtime.block_on(async {
            self.channel.close().await.unwrap();
            self.connection.close().await.unwrap();      
        });
    }

    async fn create_channel() -> (Connection, Channel, String) {
        let connection = Connection::open(&OpenConnectionArguments::default())
            .await
            .unwrap();

        let channel = connection.open_channel(None)
            .await
            .unwrap();
        channel.register_callback(DefaultChannelCallback).await.unwrap();

        let (queue_name, _, _) = channel
            .queue_declare(QueueDeclareArguments::durable_client_named("amqprs.examples.basic"))
            .await
            .unwrap()
            .unwrap();

        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                "amq.topic",
                "amqprs.example"
            ))
            .await
            .unwrap();

        (connection, channel, queue_name)
    }
}
