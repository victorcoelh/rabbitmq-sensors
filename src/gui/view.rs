use iced::theme::palette::EXTENDED_CATPPUCCIN_MACCHIATO;
use iced::widget::{button, column, container, text, Container, Row};
use iced::{Element, Length};

use super::{Message, SensorData};

pub fn main_view(sensor_data: &SensorData) -> Element<Message> {
    let mut row = Row::new()
        .spacing(20.0)
        .align_y(iced::Alignment::Center)
        .height(Length::Fill);

    for sensor in 0..sensor_data.amount_of_sensors() {
        row = row.push(sensor_widget(sensor, sensor_data));
    }

    container(row.wrap()).center(Length::Fill).into()
}

pub fn sensor_widget(index: usize, sensor_data: &SensorData) -> Container<Message> {
    let header = container(
        text(&sensor_data.names[index])
            .center()
            .color(EXTENDED_CATPPUCCIN_MACCHIATO.primary.base.text),
    )
    .center_x(250)
    .center_y(40)
    .style(|_| container::Style::from(EXTENDED_CATPPUCCIN_MACCHIATO.primary.base.color));

    let error_indication = match sensor_data.errors[index].clone() {
        None => text("No errors detected.\n"),
        Some(error) => text(format!("Sensor not within limit: {error}"))
            .color(EXTENDED_CATPPUCCIN_MACCHIATO.danger.base.color)
            .center(),
    };

    let action = button("Dismiss").on_press(Message::CloseDialogue(index));
    let contents = column!(header, error_indication, action)
        .align_x(iced::Alignment::Center)
        .spacing(10);

    container(contents)
        .height(150)
        .padding(10)
        .center_x(250)
        .style(container::rounded_box)
}
