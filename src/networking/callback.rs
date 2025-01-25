use amqprs::consumer::BlockingConsumer;
use std::sync::mpsc::Sender;

pub struct MessageConsumer {
    sender: Sender<String>,
}

impl MessageConsumer {
    pub fn new(sender: Sender<String>) -> Self {
        MessageConsumer { sender }
    }
}

impl BlockingConsumer for MessageConsumer {
    fn consume(
        &mut self, // use `&mut self` to make trait object to be `Sync`
        _channel: &amqprs::channel::Channel,
        _deliver: amqprs::Deliver,
        _basic_properties: amqprs::BasicProperties,
        content: Vec<u8>,
    ) {
        let text_data = String::from_utf8(content).unwrap();
        println!("Received: {}", text_data);

        self.sender.send(text_data).unwrap();
    }
}
