use rabbitmq_sensor::gui::state::Counter;

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
