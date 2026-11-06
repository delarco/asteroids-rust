use std::time::Duration;
use bevy::prelude::*;
use crate::{
    assets::{ShipAssets, SoundAssets},
    components::{Ship, ShipBody, ShipFire, Velocity, WrapAround},
    events::{BreakShip, FireProjectile, ShipHit},
    resources::{BulletCooldown, Lives, ShipAcceleration, ShipDeceleration, ShipRotationSpeed},
    states::InGameState,
    systems::bullet::Projectile,
};

#[derive(Component)]
pub struct AccelerationSound;

pub fn ship_setup_system(
    mut commands: Commands,
    ship_assets: Res<ShipAssets>,
    bullet_cooldown: Res<BulletCooldown>,
) {
    let fire_cooldown = Timer::new(Duration::from_millis(bullet_cooldown.0 as u64), TimerMode::Once);

    commands
        .spawn((
            Ship { fire_cooldown, },
            WrapAround,
            Velocity(Vec2::ZERO),
            Transform::IDENTITY,
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShipBody,
                Gizmo { handle: ship_assets.body.clone(), ..default() },
            ));
            parent.spawn((
                ShipFire,
                // Gizmo { handle: ship_assets.fire.clone(), ..default() },
                // Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
            ));
        });
}

pub fn ship_update_system(
    mut commands: Commands,
    ship_query: Single<(&mut Ship, &mut Velocity, &mut Transform)>,
    ship_assets: Res<ShipAssets>,
    ship_fire: Single<(Entity, Option<&Gizmo>), With<ShipFire>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    ship_acceleration: Res<ShipAcceleration>,
    ship_deceleration: Res<ShipDeceleration>,
    ship_rotation_speed: Res<ShipRotationSpeed>,
    sound_assets: Res<SoundAssets>,
    acceleration_sound: Query<Entity, With<AccelerationSound>>,
) {
    let (mut ship, mut velocity, mut transform) = ship_query.into_inner();

    // update cooldown timer
    ship.fire_cooldown.tick(time.delta());

    let mut rotation_direction = 0.0;

    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        let inc = Vec2::from_angle(transform.rotation.to_euler(EulerRot::ZYX).0);
        velocity.0 += inc * ship_acceleration.0 * time.delta_secs();

        if ship_fire.1.is_none() {
            commands.entity(ship_fire.0).insert(
                Gizmo { handle: ship_assets.fire.clone(), ..default() },
            );

        }

        if acceleration_sound.iter().len() == 0 {
            commands.spawn((
                AccelerationSound,
                AudioPlayer::new(sound_assets.acceleration.clone()),
                PlaybackSettings::LOOP,
            ));
        }
    } else {
        if ship_fire.1.is_some() {
            commands.entity(ship_fire.0).remove::<Gizmo>();
        }

        for entity in acceleration_sound {
            commands.entity(entity).despawn();
        }
    }

    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        rotation_direction += -1.0;
    }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        rotation_direction += 1.0;
    }

    if keys.any_pressed([KeyCode::Space, KeyCode::KeyQ]) && ship.fire_cooldown.is_finished() {
        ship.fire_cooldown.reset();
        commands.trigger(FireProjectile(Projectile::BULLET));
    }

    if keys.any_pressed([KeyCode::Enter, KeyCode::KeyE]) && ship.fire_cooldown.is_finished() {
        ship.fire_cooldown.reset();
        commands.trigger(FireProjectile(Projectile::BOMB));
    }

    transform.rotate_z(rotation_direction * ship_rotation_speed.0 * time.delta_secs());

    let dec = Vec2::new(-velocity.0.x, -velocity.0.y).normalize_or_zero();
    velocity.0 += dec * ship_deceleration.0 * time.delta_secs();
}

pub fn on_ship_hit(
    _trigger: On<ShipHit>,
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut next_game_state: ResMut<NextState<InGameState>>,
) {
    if lives.0 > 0 {
        lives.0 -= 1;
        next_game_state.set(InGameState::Respawning);
    } else {
        next_game_state.set(InGameState::Gameover);
    }

    commands.trigger(BreakShip);
}
