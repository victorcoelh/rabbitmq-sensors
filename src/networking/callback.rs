use amqprs::consumer::BlockingConsumer;

#[derive(Default)]
pub struct MessageConsumer { }

impl BlockingConsumer for MessageConsumer {
    fn consume(
            &mut self, // use `&mut self` to make trait object to be `Sync`
            _channel: &amqprs::channel::Channel,
            _deliver: amqprs::Deliver,
            _basic_properties: amqprs::BasicProperties,
            content: Vec<u8>,
        ) {
        println!("{}", String::from_utf8(content).unwrap());
    }
}
