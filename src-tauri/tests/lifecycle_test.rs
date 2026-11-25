use lazyme_lib::lifecycle::{
    decide_event_action, decide_startup_action, LifecycleAction, LifecycleEvent, SecretProbe,
};

struct FakeSecretProbe {
    result: Result<bool, String>,
}

impl SecretProbe for FakeSecretProbe {
    fn has_api_key(&self) -> Result<bool, String> {
        self.result.clone()
    }
}

#[test]
fn opens_settings_on_first_launch_when_no_api_key_is_configured() {
    let probe = FakeSecretProbe { result: Ok(false) };
    assert_eq!(
        decide_startup_action(&probe),
        LifecycleAction::ShowSettingsWindow
    );
}

#[test]
fn starts_hidden_in_tray_when_api_key_already_exists() {
    let probe = FakeSecretProbe { result: Ok(true) };
    assert_eq!(
        decide_startup_action(&probe),
        LifecycleAction::StartHiddenInTray
    );
}

#[test]
fn opens_settings_if_credential_store_check_errors() {
    let probe = FakeSecretProbe {
        result: Err("keyring unavailable".to_string()),
    };
    assert_eq!(
        decide_startup_action(&probe),
        LifecycleAction::ShowSettingsWindow
    );
}

#[test]
fn hides_main_window_to_tray_when_close_is_requested() {
    assert_eq!(
        decide_event_action(LifecycleEvent::WindowCloseRequested),
        LifecycleAction::HideMainWindow
    );
}

#[test]
fn shows_settings_window_when_tray_open_or_second_instance_triggers() {
    assert_eq!(
        decide_event_action(LifecycleEvent::TrayOpenClicked),
        LifecycleAction::ShowSettingsWindow
    );
    assert_eq!(
        decide_event_action(LifecycleEvent::SecondInstanceLaunched),
        LifecycleAction::ShowSettingsWindow
    );
}

#[test]
fn exits_app_when_tray_quit_is_clicked() {
    assert_eq!(
        decide_event_action(LifecycleEvent::TrayQuitClicked),
        LifecycleAction::ExitApp
    );
}
