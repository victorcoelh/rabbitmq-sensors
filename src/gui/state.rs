use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Message {
    AddSensor(String),
    SensorError(String, String),
    CloseDialogue(usize),
}

pub struct SensorData {
    pub receiver: Arc<Mutex<Receiver<String>>>,
    pub names: Vec<String>,
    pub errors: Vec<Option<String>>,
}

impl SensorData {
    pub fn new(receiver: Arc<Mutex<Receiver<String>>>) -> Self {
        SensorData {
            receiver,
            names: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor_name: String) {
        self.names.push(sensor_name);
        self.errors.push(None);
    }

    pub fn get_index_from_name(&self, name: &str) -> Option<usize> {
        self.names
            .iter()
            .position(|sensor_name| sensor_name == name)
    }

    pub fn amount_of_sensors(&self) -> usize {
        self.names.len()
    }
}
