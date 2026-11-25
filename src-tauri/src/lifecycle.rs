/// Checks whether an API key is already configured in credential storage.
pub trait SecretProbe {
    fn has_api_key(&self) -> Result<bool, String>;
}

/// Lifecycle events emitted by the desktop app shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    AppStarted,
    WindowCloseRequested,
    TrayOpenClicked,
    TrayQuitClicked,
    SecondInstanceLaunched,
}

/// Action the desktop shell should take in response to a lifecycle event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleAction {
    ShowSettingsWindow,
    StartHiddenInTray,
    HideMainWindow,
    ExitApp,
}

/// Decides the shell action for a lifecycle event without coupling to Tauri runtime APIs.
pub fn decide_action(event: LifecycleEvent, secrets: &dyn SecretProbe) -> LifecycleAction {
    match event {
        LifecycleEvent::AppStarted => match secrets.has_api_key() {
            Ok(true) => LifecycleAction::StartHiddenInTray,
            // Default to showing Settings when no key exists or probing fails,
            // so the user is never stranded without access to configuration.
            Ok(false) | Err(_) => LifecycleAction::ShowSettingsWindow,
        },
        LifecycleEvent::WindowCloseRequested => LifecycleAction::HideMainWindow,
        LifecycleEvent::TrayOpenClicked | LifecycleEvent::SecondInstanceLaunched => {
            LifecycleAction::ShowSettingsWindow
        }
        LifecycleEvent::TrayQuitClicked => LifecycleAction::ExitApp,
    }
}
