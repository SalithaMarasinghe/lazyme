pub mod lifecycle;
pub mod secrets;

use lifecycle::{decide_action, LifecycleAction, LifecycleEvent, SecretProbe};
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
    let probe = KeyringSecretProbe;
    let has_api_key = probe.has_api_key().unwrap_or(false);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let probe = KeyringSecretProbe;

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let action = decide_action(LifecycleEvent::SecondInstanceLaunched, &KeyringSecretProbe);
            apply_action(app, action);
        }))
        .invoke_handler(tauri::generate_handler![get_app_status])
        .setup(move |app| {
            let open_item = MenuItem::with_id(app, "open", "Open LazyMe", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

            let mut tray_builder = TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .tooltip("LazyMe")
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        let action =
                            decide_action(LifecycleEvent::TrayOpenClicked, &KeyringSecretProbe);
                        apply_action(app, action);
                    }
                    "quit" => {
                        let action =
                            decide_action(LifecycleEvent::TrayQuitClicked, &KeyringSecretProbe);
                        apply_action(app, action);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let action =
                            decide_action(LifecycleEvent::TrayOpenClicked, &KeyringSecretProbe);
                        apply_action(tray.app_handle(), action);
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            tray_builder.build(app)?;

            let initial_action = decide_action(LifecycleEvent::AppStarted, &probe);
            apply_action(app.handle(), initial_action);
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let action =
                        decide_action(LifecycleEvent::WindowCloseRequested, &KeyringSecretProbe);
                    apply_action(window.app_handle(), action);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running LazyMe");
}
