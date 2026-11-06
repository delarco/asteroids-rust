use bevy::state::state::States;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    Loading,
    AudioUnlock,
    Intro,
    Menu,
    InGame,
    Credits,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum InGameState {
    #[default]
    None,
    Start,
    Running,
    Paused,
    Respawning,
    Gameover,
}
