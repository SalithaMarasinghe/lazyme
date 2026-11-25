pub mod lifecycle;
pub mod secrets;

use lifecycle::{decide_event_action, decide_startup_action, LifecycleAction, LifecycleEvent, SecretProbe};
use secrets::KeyringSecretProbe;
use serde::Serialize;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatus {
    pub has_api_key: bool,
}

#[tauri::command]
fn get_app_status() -> AppStatus {
    let has_api_key = KeyringSecretProbe.has_api_key().unwrap_or(false);
    AppStatus { has_api_key }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn apply_action(app: &AppHandle, action: LifecycleAction) {
    match action {
        LifecycleAction::ShowSettingsWindow => show_main_window(app),
        LifecycleAction::StartHiddenInTray => {}
        LifecycleAction::HideMainWindow => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        LifecycleAction::ExitApp => app.exit(0),
    }
}

fn dispatch_event(app: &AppHandle, event: LifecycleEvent) {
    apply_action(app, decide_event_action(event));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            dispatch_event(app, LifecycleEvent::SecondInstanceLaunched);
        }))
        .invoke_handler(tauri::generate_handler![get_app_status])
        .setup(|app| {
            let open_item = MenuItem::with_id(app, "open", "Open LazyMe", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

            let mut tray_builder = TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .tooltip("LazyMe")
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => dispatch_event(app, LifecycleEvent::TrayOpenClicked),
                    "quit" => dispatch_event(app, LifecycleEvent::TrayQuitClicked),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        dispatch_event(tray.app_handle(), LifecycleEvent::TrayOpenClicked);
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            tray_builder.build(app)?;

            apply_action(app.handle(), decide_startup_action(&KeyringSecretProbe));
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    dispatch_event(window.app_handle(), LifecycleEvent::WindowCloseRequested);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LazyMe");
}
