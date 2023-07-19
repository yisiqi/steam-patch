pub mod device_ally;
use std::fs;
use regex::Regex;
use device_ally::DeviceAlly;
use crate::server::SettingsRequest;

pub struct Patch {
    pub text_to_find: String,
    pub replacement_text: String,
}

pub trait Device {
    fn update_settings(&self, request: SettingsRequest);
    fn set_tdp(&self, tdp: i8);
    fn get_patches(&self) -> Vec<Patch>;
}

pub fn create_device() -> Option<Box<dyn Device>> {
    let device_name = get_device_name();

    match device_name.trim().as_ref() {
        "AMD Ryzen Z1 Extreme ASUSTeK COMPUTER INC. RC71L" => Some(Box::new(DeviceAlly)),
        _ => None,
    }
}

fn get_device_name() -> String {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").expect("Unknown");

    let model_re = Regex::new(r"model name\s*:\s*(.*)").unwrap();
    let model = model_re.captures_iter(&cpuinfo).next().unwrap()[1].trim().to_string();

    let board_vendor = fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
        .expect("Unknown").trim().to_string();

    let board_name = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name")
        .expect("Unknown").trim().to_string();

    format!("{} {} {}", model, board_vendor, board_name)
}