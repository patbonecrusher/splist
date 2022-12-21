#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::{available_ports, SerialPortInfo};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn listport() -> String {
    match available_ports() {
        Ok(ports) => {
            //format!("Found {} ports:", ports.len())
            match ports.len() {
                0 => format!("No ports found."),
                1 => format!("Found 1 port:"),
                n => format!("Found {} ports:", n),
            }
        }
        Err(e) => {
            format!("{:?}", e)
        }
    }
}

#[tauri::command]
fn port_count() -> usize {
    match available_ports() {
        Ok(ports) => {
            //format!("Found {} ports:", ports.len())
            ports.len()
        }
        Err(_e) => {
            0
        }
    }
}

#[derive(serde::Serialize)]
struct Data {
  some_key: String,
  some_other_key: String
}

#[tauri::command]
fn port_at_index(idx: usize) -> Data {
    match available_ports() {
        Ok(ports) => {
            Data {
                some_key: ports[idx].port_name.to_string(),
                some_other_key: format!("{}", idx),
            }
        }
        Err(_e) => {
            Data {
                some_key: "ooops".to_string(),
                some_other_key: format!("{}", idx),
            }
        }
    }

}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, listport, port_count, port_at_index])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
