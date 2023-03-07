use bevy::prelude::*;

#[derive(States, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub enum AppState {
    #[default]
    SplashScreen,
    MainMenu,
    SettingsMenu,
    GameRunning,
}
