#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serialport::{available_ports};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn listport() -> String {
    match available_ports() {
        Ok(ports) => {
            format!("Found {} ports:", ports.len())
            // match ports.len() {
            //     0 => format!("No ports found."),
            //     1 => format!("Found 1 port:"),
            //     n => format!("Found {} ports:", n),
            // };
        }
        Err(e) => {
            format!("{:?}", e)
        }
    }
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, listport])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
