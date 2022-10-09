#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;

use tauri::{AppHandle, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
fn evoke_main_command(window: tauri::Window) {
    println!("I was invoked from JS!");
    let brisk_window = window.get_window("brisk_main").unwrap();
    brisk_window.show().unwrap();
    // TODO 聚焦到窗口
    brisk_window.set_focus().unwrap();
}

#[tauri::command]
fn add_task(content: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("../tasks.txt")
        .expect("Error while opening the tasks file");
    writeln!(file, "{}", content).expect("Error while writing in the tasks file");
}

fn make_tray() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("toggle".to_string(), "Hide"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
    return SystemTray::new().with_menu(menu);
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        if id.as_str() == "quit" {
            process::exit(0);
        }
        if id.as_str() == "toggle" {
            let window = app.get_window("brisk_main").unwrap();
            let menu_item = app.tray_handle().get_item("toggle");
            if window.is_visible().unwrap() {
                window.hide();
                menu_item.set_title("Show");
            } else {
                window.show();
                window.center();
                menu_item.set_title("Hide");
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![evoke_main_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
