use iced::widget::{button, column, text, Column};

use super::messages::Message;

#[derive(Default)]
pub struct Counter {
    value: i64
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value +=1,
            Message::Decrement => self.value -=1,
        }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}
