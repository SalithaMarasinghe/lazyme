use lazyme_lib::lifecycle::{decide_action, LifecycleAction, LifecycleEvent, SecretProbe};

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
    let action = decide_action(LifecycleEvent::AppStarted, &probe);
    assert_eq!(action, LifecycleAction::ShowSettingsWindow);
}

#[test]
fn starts_hidden_in_tray_when_api_key_already_exists() {
    let probe = FakeSecretProbe { result: Ok(true) };
    let action = decide_action(LifecycleEvent::AppStarted, &probe);
    assert_eq!(action, LifecycleAction::StartHiddenInTray);
}

#[test]
fn opens_settings_if_credential_store_check_errors() {
    let probe = FakeSecretProbe {
        result: Err("keyring unavailable".to_string()),
    };
    let action = decide_action(LifecycleEvent::AppStarted, &probe);
    assert_eq!(action, LifecycleAction::ShowSettingsWindow);
}

#[test]
fn hides_main_window_to_tray_when_close_is_requested() {
    let probe = FakeSecretProbe { result: Ok(true) };
    let action = decide_action(LifecycleEvent::WindowCloseRequested, &probe);
    assert_eq!(action, LifecycleAction::HideMainWindow);
}

#[test]
fn shows_settings_window_when_tray_open_or_second_instance_triggers() {
    let probe = FakeSecretProbe { result: Ok(true) };
    assert_eq!(
        decide_action(LifecycleEvent::TrayOpenClicked, &probe),
        LifecycleAction::ShowSettingsWindow
    );
    assert_eq!(
        decide_action(LifecycleEvent::SecondInstanceLaunched, &probe),
        LifecycleAction::ShowSettingsWindow
    );
}

#[test]
fn exits_app_when_tray_quit_is_clicked() {
    let probe = FakeSecretProbe { result: Ok(true) };
    let action = decide_action(LifecycleEvent::TrayQuitClicked, &probe);
    assert_eq!(action, LifecycleAction::ExitApp);
}
