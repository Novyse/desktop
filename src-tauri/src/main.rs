// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent, State, Window,
};
use std::sync::Mutex;

// State to track if the app should quit when window is closed
#[derive(Default)]
struct AppState {
    should_quit: Mutex<bool>,
}

#[tauri::command]
fn show_window(window: Window) {
    window.show().unwrap();
    window.set_focus().unwrap();
}

#[tauri::command]
fn hide_window(window: Window) {
    window.hide().unwrap();
}

#[tauri::command]
fn quit_app(app_handle: AppHandle, state: State<AppState>) {
    *state.should_quit.lock().unwrap() = true;
    app_handle.exit(0);
}

fn create_system_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap() {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .system_tray(create_system_tray())
        .on_system_tray_event(handle_system_tray_event)
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                let app_state: State<AppState> = event.window().state();
                let should_quit = *app_state.should_quit.lock().unwrap();
                
                if !should_quit {
                    // Hide the window instead of closing the app
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![show_window, hide_window, quit_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}