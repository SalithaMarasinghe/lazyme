/// Checks whether an API key is already configured in credential storage.
pub trait SecretProbe {
    fn has_api_key(&self) -> Result<bool, String>;
}

/// Runtime lifecycle events emitted after app startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    WindowCloseRequested,
    TrayOpenClicked,
    TrayQuitClicked,
    SecondInstanceLaunched,
}

/// Action the desktop shell should take in response to startup or a lifecycle event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleAction {
    ShowSettingsWindow,
    StartHiddenInTray,
    HideMainWindow,
    ExitApp,
}

/// Decides initial window visibility at startup without coupling to Tauri runtime APIs.
pub fn decide_startup_action(secrets: &dyn SecretProbe) -> LifecycleAction {
    match secrets.has_api_key() {
        Ok(true) => LifecycleAction::StartHiddenInTray,
        // Default to showing Settings when no key exists or probing fails,
        // so the user is never stranded without access to configuration.
        Ok(false) | Err(_) => LifecycleAction::ShowSettingsWindow,
    }
}

/// Decides the shell action for a post-startup lifecycle event.
pub fn decide_event_action(event: LifecycleEvent) -> LifecycleAction {
    match event {
        LifecycleEvent::WindowCloseRequested => LifecycleAction::HideMainWindow,
        LifecycleEvent::TrayOpenClicked | LifecycleEvent::SecondInstanceLaunched => {
            LifecycleAction::ShowSettingsWindow
        }
        LifecycleEvent::TrayQuitClicked => LifecycleAction::ExitApp,
    }
}
