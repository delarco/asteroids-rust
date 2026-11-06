use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::RngExt;

use crate::{
    ASTEROID_SPAWN_IGNORE_RECT, BOUNDS_RECT, assets::{AsteroidAssets, SoundAssets}, components::{Asteroid, Velocity, WrapAround}, events::{BreakAsteroid, NextLevel, SpawnAsteroid}, resources::AsteroidVelocityMinMax
};

struct AsteroidSpawnValues {
    position: Vec2,
    velocity: Vec2,
    rotation: Quat,
    asset_index: usize,
}

fn asteroid_spawn_randomize(
    assets_len: usize,
    from_point: Option<Vec2>,
    ignore_ship_area: bool,
    asteroid_velocity_min_max: &Res<AsteroidVelocityMinMax>,
)-> AsteroidSpawnValues  {
    let mut rng = rand::rng();

    let mut position = Vec2::new(0., 0.);

    if let Some(point) = from_point {
        position = point;
    }

    if ignore_ship_area {
        while ASTEROID_SPAWN_IGNORE_RECT.contains(position) {
            position.x = rng.random_range(-BOUNDS_RECT.x..BOUNDS_RECT.x);
            position.y = rng.random_range(-BOUNDS_RECT.y..BOUNDS_RECT.y);
        }
    }

    AsteroidSpawnValues {
        position,
        velocity: Vec2::new(
            rng.random_range(asteroid_velocity_min_max.0..asteroid_velocity_min_max.1)
                * if rand::random::<bool>() { 1.0 } else { -1.0 },
            rng.random_range(asteroid_velocity_min_max.0..asteroid_velocity_min_max.1)
                * if rand::random::<bool>() { 1.0 } else { -1.0 },
        ),
        rotation: Quat::from_rotation_z(rng.random_range(0.0..TAU)),
        asset_index: rng.random_range(0..assets_len),
    }
}

pub fn on_asteroid_spawn(
    trigger: On<SpawnAsteroid>,
    mut commands: Commands,
    asteroid_assets: Res<AsteroidAssets>,
    asteroid_velocity_min_max: Res<AsteroidVelocityMinMax>,
) {
    let asteroid_rng = asteroid_spawn_randomize(
        asteroid_assets.0.len(),
        trigger.from_point,
        trigger.ignore_ship_area,
        &asteroid_velocity_min_max,
    );

    commands.spawn((
        Asteroid(trigger.size),
            Gizmo {
            handle: asteroid_assets.0[asteroid_rng.asset_index].clone(),
            ..default()
        },
        WrapAround,
        Velocity(asteroid_rng.velocity),
        Transform::from_xyz(asteroid_rng.position.x, asteroid_rng.position.y, 0.)
            .with_rotation(asteroid_rng.rotation)
            .with_scale(Vec3::splat(trigger.size as f32)),
    ));
}

pub fn on_asteroid_break(
    trigger: On<BreakAsteroid>,
    mut commands: Commands,
    asteroids: Query<(&Asteroid, &Transform)>,
    asteroid_assets: Res<AsteroidAssets>,
    asteroid_velocity_min_max: Res<AsteroidVelocityMinMax>,
    sound_assets: Res<SoundAssets>,
) {
    commands.spawn((
        AudioPlayer::new(sound_assets.asteroid_hit.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(0.5),
            ..default()
        },
    ));

    if let Ok((breaking_asteroid, transform)) = asteroids.get(trigger.0) {
        if breaking_asteroid.0 > 1 {

            let size = breaking_asteroid.0 - 1;

            for _ in 0..2 {
                let asteroid_rng = asteroid_spawn_randomize(
                    asteroid_assets.0.len(),
                    Some(transform.translation.truncate()),
                    false,
                    &asteroid_velocity_min_max,
                );

                commands.spawn((
                    Asteroid(size),
                        Gizmo {
                        handle: asteroid_assets.0[asteroid_rng.asset_index].clone(),
                        ..default()
                    },
                    WrapAround,
                    Velocity(asteroid_rng.velocity),
                    Transform::from_xyz(asteroid_rng.position.x, asteroid_rng.position.y, 0.)
                        .with_rotation(asteroid_rng.rotation)
                        .with_scale(Vec3::splat(size as f32)),
                ));
            }
        }

        commands.entity(trigger.0).despawn();

        if breaking_asteroid.0 == 1 && asteroids.iter().len() == 1 {
            commands.trigger(NextLevel);
        }

    }
}
