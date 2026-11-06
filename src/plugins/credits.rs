use std::{f32::consts::{FRAC_PI_4, TAU}, time::Duration};
use rand::{RngExt, seq::IndexedRandom}; 
use bevy::prelude::*;

use crate::{
    BOUNDS_HALF_RECT,
    assets::{FireworkAssets, SoundAssets},
    components::Velocity,
    states::AppState,
    systems::movement::movement_system
};

pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), setup);
        app.add_systems(OnExit(AppState::Credits), cleanup);
        app.add_systems(Update, 
            (
                shoot_system,
                explode_system,
                despawn_system,
                movement_system,
                check_key_system,
            ).run_if(in_state(AppState::Credits))
        );
    }
}

#[derive(Component)]
struct CreditsText;

fn setup(mut commands: Commands) {
    commands.spawn((
        CreditsText,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Asteroids"),
            TextFont { font_size: FontSize::Px(40.), ..default() },
            // TextColor(Color::srgb(2.0, 2.0, 4.0)),
            TextColor(Color::srgb(0.7, 0.7, 1.0)),
        ));
        parent.spawn((
            Text::new("\nCONGRATULATIONS CAPTAIN!"),
            TextFont { font_size: FontSize::Px(20.), ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("THE DEBRIS FIELD IS CLEAR."),
            TextFont { font_size: FontSize::Px(20.), ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("GALAXY SECURED."),
            TextFont { font_size: FontSize::Px(20.), ..default() },
            TextColor(Color::WHITE),
        ));

        parent.spawn((
            Text::new("\n[ PRESS ANY KEY TO RE-ENTER HYPERSPACE ]"),
            TextFont { font_size: FontSize::Px(16.), ..default() },
            TextColor(Color::srgb(0.7, 1.0, 0.7)),
            // TextColor(Color::srgb(2.0, 3.0, 2.0)),
            // TextColor(Color::srgb(0.0, 1.0, 1.0) * 3.0), 
        ));
    });

    let mut shoot_timer = Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating);
    shoot_timer.set_elapsed(Duration::from_secs_f32(0.7)); 
    commands.spawn(FireworkShootTimer(shoot_timer));
}

#[derive(Component)]
struct FireworkShootTimer(Timer);

#[derive(Component)]
struct Firework;

#[derive(Component)]
struct FireworkExplodeTimer(Timer);

#[derive(Component)]
struct FireworkDespawnTimer(Timer);

fn shoot_system(
    mut commands: Commands,
    mut firework_timer: Single<&mut FireworkShootTimer>,
    time: Res<Time>,
    fierwork_assets: Res<FireworkAssets>,
    sound_assets: Res<SoundAssets>,
) {
     let mut rng = rand::rng();

    firework_timer.0.tick(time.delta());

    if firework_timer.0.just_finished() {
        let angle = rng.random_range(FRAC_PI_4..=(FRAC_PI_4 * 3.));
        let explode_secs = rng.random_range(1.5..3.0);

        commands.spawn((
            Firework,
            Gizmo { handle: fierwork_assets.0.choose(&mut rng).unwrap().clone(), ..default() },
            Transform::from_xyz(0., -BOUNDS_HALF_RECT.y + 20., 0.),
            Velocity(Vec2::from_angle(angle) * 250.),
            FireworkExplodeTimer(Timer::new(Duration::from_secs_f32(explode_secs), TimerMode::Once))
        ));

        commands.spawn((
            AudioPlayer::new(sound_assets.firework_launch.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(0.3),
                ..default()
            },
        ));
    }
}

fn explode_system(
    mut commands: Commands,
    time: Res<Time>,
    fierwork_assets: Res<FireworkAssets>,
    mut fireworks: Query<(Entity, &Transform, &mut FireworkExplodeTimer), With<Firework>>,
    sound_assets: Res<SoundAssets>,
) {
    let mut rng = rand::rng();

    for (entity, transform, mut timer) in &mut fireworks {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            for _ in 0..20 {
                let angle = rng.random_range(0.0..=TAU);
                let despawn_secs = rng.random_range(2.5..5.0);

                commands.spawn((
                    Firework,
                    Gizmo { handle: fierwork_assets.0.choose(&mut rng).unwrap().clone(), ..default() },
                    Transform::from_translation(transform.translation),
                    Velocity(Vec2::from_angle(angle) * rng.random_range(90.0..150.)),
                    FireworkDespawnTimer(Timer::new(Duration::from_secs_f32(despawn_secs), TimerMode::Once)),
                ));
            }

            commands.entity(entity).despawn();

            commands.spawn((
            AudioPlayer::new(sound_assets.firework_explosion.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    // volume: bevy::audio::Volume::Linear(0.5),
                    ..default()
                },
            ));
        }
    }
}

fn despawn_system(
    mut commands: Commands,
    mut fireworks: Query<(Entity, &mut FireworkDespawnTimer), With<Firework>>,
    time: Res<Time>,
) {
    for (entity, mut firework) in &mut fireworks {
        firework.0.tick(time.delta());
        if firework.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn check_key_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keys.get_just_pressed().len() > 0 {
        next_app_state.set(AppState::Menu);
    }
}

fn cleanup(
    mut commands: Commands,
    text_node: Single<Entity, With<CreditsText>>,
    firework_shoot_timer: Single<Entity, With<FireworkShootTimer>>,
    fireworks: Query<Entity, With<Firework>>,
) {
    commands.entity(*text_node).despawn();
    
    commands.entity(*firework_shoot_timer).despawn();

    for entity in fireworks {
        commands.entity(entity).despawn();
    }
}
