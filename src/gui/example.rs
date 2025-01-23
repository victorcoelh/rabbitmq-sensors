use iced::{alignment, Element, Length};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};

#[derive(Debug, Clone)]
pub enum Message{
    InputValue(String),
    Submitted,
}

pub struct GroceryList {
    grocery_items: Vec<String>,
    input_value: String
}

impl Default for GroceryList {
    fn default() -> Self {
        let grocery_items = vec!(
            "Eggs".to_string(),
            "Milk".to_string(),
            "Flour".to_string(),
        );

        Self { grocery_items, input_value: String::default() }
    }
}

pub fn update(groceries: &mut GroceryList, message: Message) {
    match message {
        Message::InputValue(value) => groceries.input_value = value,
        Message::Submitted => {
            groceries.grocery_items.push(groceries.input_value.clone());
            groceries.input_value = String::default()
        }
    }
}

pub fn view(groceries: &GroceryList) -> Element<Message> {
    let text_widget = row!(
        text_input("Input grocery item", &groceries.input_value)
            .on_input(|value| Message::InputValue(value))
            .on_submit(Message::Submitted),
        button("Submit")
            .on_press(Message::Submitted)
    );

    let column_widget = column!(
            items_list_view(&groceries.grocery_items),
            text_widget
        )
        .align_x(iced::Alignment::Center);

    container(column_widget)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
}

fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill);

    for value in items {
        column = column.push(text(value.clone()));
    }

    scrollable(
        container(column)
            .height(250.0)
            .width(300.0)
        )
        .into()
}
