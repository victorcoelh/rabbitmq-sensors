use iced::widget::{button, column, container, text, Container, Row};
use iced::{alignment, Element, Length};

use super::{Message, SensorData};

pub fn main_view(sensor_data: &SensorData) -> Element<Message> {
    let mut row = Row::new()
        .spacing(20.0)
        .align_y(iced::Alignment::Center)
        .height(Length::Fill);

    for sensor in 0..sensor_data.amount_of_sensors() {
        row = row.push(sensor_widget(sensor, sensor_data));
    }

    container(row.wrap())
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
}

pub fn sensor_widget(index: usize, sensor_data: &SensorData) -> Container<Message> {
    let error_indication = match sensor_data.errors[index].clone() {
        Some(error) => text(format!(
            "WARNING! Sensor reading not within limits: {error}"
        )),
        None => text("No errors detected."),
    };

    let column = column!(text(&sensor_data.names[index]), error_indication);

    container(column)
        .padding(10)
        .center(200)
        .style(container::rounded_box)
}
