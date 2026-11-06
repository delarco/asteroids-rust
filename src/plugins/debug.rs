use bevy::prelude::*;

use crate::{
    components::{Asteroid, Bullet},
    plugins::{
        game::PauseMenuUi,
        menu::{MenuUi, SettingsMenuUi}
    },
    resources::{DebugTool, LevelNumber, Lives},
    states::{AppState, InGameState},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct DebugText;

fn setup(mut commands: Commands) {
    commands.spawn((
        DebugText,
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(32),
            left: px(12),
            ..default()
        },
    ));
}

fn update(
    mut text: Single<&mut Text, With<DebugText>>,
    debug_tool: Res<DebugTool>,
    bullets: Query<&Bullet>,
    asteroids: Query<&Asteroid>,
    app_state: Res<State<AppState>>,
    in_game_state: Res<State<InGameState>>,
    main_menu_ui: Query<&MenuUi>,
    settings_ui: Query<&SettingsMenuUi>,
    pause_ui: Query<&PauseMenuUi>,
    level_number: Res<LevelNumber>,
    lives: Res<Lives>,
) {
    if !debug_tool.0 {
        text.0 = "".to_string();
        return;
    }

    text.0 = format!("[debug]\n",);

    let app_state_name = match app_state.get() {
        AppState::Loading => "Loading".to_string(),
        AppState::Intro => "Intro".to_string(),
        AppState::AudioUnlock => "AudioUnlock".to_string(),
        AppState::Menu => "Menu".to_string(),
        AppState::InGame => "InGame".to_string(),
        AppState::Credits => "Credits".to_string(),
    };
    text.push_str(&format!("app state: {}\n", app_state_name,));
    
    let in_game_state_name = match in_game_state.get() {
        InGameState::None => "None".to_string(),
        InGameState::Start => "Start".to_string(),
        InGameState::Running => "Running".to_string(),
        InGameState::Paused => "Paused".to_string(),
        InGameState::Respawning => "Respawning".to_string(),
        InGameState::Gameover => "Gameover".to_string(),
    };
    text.push_str(&format!("game state: {}\n", in_game_state_name,));
    text.push_str(&format!("asteroids: {:.2}\n", asteroids.iter().len(),));
    text.push_str(&format!("bullets: {:.2}\n", bullets.iter().len(),));
    text.push_str(&format!("main_menu_ui: {}\n", main_menu_ui.iter().len()));
    text.push_str(&format!("settings_ui: {}\n", settings_ui.iter().len(),));
    text.push_str(&format!("pause_ui: {}\n", pause_ui.iter().len(),));
    text.push_str(&format!("level_number: {}\n", level_number.0,));
    text.push_str(&format!("lives: {}\n", lives.0,));
}
