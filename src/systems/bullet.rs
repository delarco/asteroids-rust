use std::{f32::consts::TAU, time::Duration};
use bevy::{audio::Volume, prelude::*};

use crate::{
    assets::{BulletAssets, SoundAssets},
    components::{Bullet, Ship, Velocity, WrapAround},
    events::FireProjectile,
    resources::{BombProjectiles, BulletDespawn, BulletVelocity},
};

pub enum Projectile {
    BULLET,
    BOMB,
}

pub fn on_fire_projectile(
    trigger: On<FireProjectile>,
    mut commands: Commands,
    ship_transform: Single<&Transform, With<Ship>>,
    bullet_assets: Res<BulletAssets>,
    bullet_despawn: Res<BulletDespawn>,
    bullet_velocity: Res<BulletVelocity>,
    bomb_projectiles: Res<BombProjectiles>,
    sound_assets: Res<SoundAssets>,
) {
    let projectiles_num = match trigger.0 {
        Projectile::BULLET => 1,
        Projectile::BOMB => bomb_projectiles.0,
    };

    let angle_step = TAU / projectiles_num as f32;

    for projectile_index in 0..projectiles_num {
        let angle = match trigger.0 {
            Projectile::BULLET => ship_transform.rotation.to_euler(EulerRot::ZYX).0,
            Projectile::BOMB => angle_step * projectile_index as f32, //rng.random_range(0.0..=TAU),
        };

        let despawn_timer = Timer::new(Duration::from_millis(bullet_despawn.0 as u64), TimerMode::Once);
        let velocity = Vec2::from_angle(angle) * bullet_velocity.0;

        commands.spawn((
            Bullet { despawn_timer },
            Gizmo { handle: bullet_assets.0.clone(), ..default() },
            WrapAround,
            Velocity(velocity),
            Transform::from_translation(ship_transform.translation)
                .with_rotation(ship_transform.rotation),
        ));
    }

    let sound_effect_handle = match trigger.0 {
        Projectile::BULLET => sound_assets.fire.clone(),
        Projectile::BOMB => sound_assets.fire.clone(),
    };

    commands.spawn((
        AudioPlayer::new(sound_effect_handle),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(0.3),
            ..default()
        }
    ));
}

pub fn bullet_update_system(
    mut commands: Commands,
    bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
) {
    for (entity, mut bullet) in bullets {
        bullet.despawn_timer.tick(time.delta());
        if bullet.despawn_timer.is_finished() {
            commands.entity(entity).despawn();
            return;
        }
    }
}
