use bevy::prelude::*;

use crate::{assets::SoundAssets, states::AppState};

pub struct AudioUnlockPlugin;

impl Plugin for AudioUnlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AudioUnlock), setup);
        app.add_systems(Update, update.run_if(in_state(AppState::AudioUnlock)));
    }
}

#[derive(Component)]
struct AudioUnlockUi;

fn setup(mut commands: Commands) {
    commands.spawn((
        AudioUnlockUi,
        DespawnOnExit(AppState::AudioUnlock),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            Text::new("Click or press any key to start..."),
            TextFont { font_size: FontSize::Px(24.), ..default() },
            TextColor(Color::srgb(0.8, 0.8, 1.0)),
            TextShadow {
                offset: Vec2::new(3.0, 3.0),
                color: Color::srgb(0.1, 0.1, 0.5),
            },
        )]
    ));
}

fn update(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    sound_assets: Res<SoundAssets>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if mouse_buttons.get_just_pressed().len() > 0 || keys.get_just_pressed().len() > 0 {
        commands.spawn((
            AudioPlayer::new(sound_assets.click.clone()),
            PlaybackSettings::DESPAWN,
        ));

        next_app_state.set(AppState::Intro);
    }
}
