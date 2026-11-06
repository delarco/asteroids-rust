use bevy::{
    audio::Volume, color::palettes::css::GRAY, prelude::*, ui_widgets::Button,
};

use crate::assets::SoundAssets;

pub const BUTTON_BORDER_COLOR: Color = Color::linear_rgb(0.05, 0.05, 0.05);

pub fn button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(200),
            height: px(65),
            border: px(5),
            border_radius: BorderRadius::MAX,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::from(BUTTON_BORDER_COLOR)
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
        Children [
            Text(label)
            TextFont { font_size: px(16.0) }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
            Pickable::IGNORE
        ]
        on(on_hover_enter)
        on(on_hover_exit)
        on(on_pointer_press)
    }
}

pub fn on_hover_enter(
    trigger: On<PointerEnter>,
    mut commands: Commands,
        sound_assets: Res<SoundAssets>,
) {
    commands
        .entity(trigger.entity)
        .insert(BorderColor::from(GRAY));

    commands.spawn((
        AudioPlayer::new(sound_assets.mouse_hover.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(0.2),
            ..default()
        }
    ));
}

pub fn on_hover_exit(trigger: On<PointerOut>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(BorderColor::from(BUTTON_BORDER_COLOR));
}

pub fn on_pointer_press(
    _event: On<PointerPress>,
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
) {
    commands.spawn((
        AudioPlayer::new(sound_assets.click.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
