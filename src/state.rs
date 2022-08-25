use serde::Serialize;
use serialport::{self, SerialPort};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct SerialportState {
    // plugin state, configuration fields
    pub serialports: Arc<Mutex<HashMap<String, SerialportInfo>>>,
}
pub struct SerialportInfo {
    pub serialport: Box<dyn SerialPort>,
    pub is_reading: bool,
}

#[derive(Serialize, Clone)]
pub struct InvokeResult {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct ReadData<'a> {
    pub data: &'a [u8],
    pub size: usize,
}
