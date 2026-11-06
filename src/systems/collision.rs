use bevy::prelude::*;

use crate::{
    components::{Asteroid, Bullet, Ship},
    events::{BreakAsteroid, ShipHit},
    resources::DebugTool,
};

pub fn collision_update_system(
    mut commands: Commands,
    mut gizmos: Gizmos,
    debug_tool: Res<DebugTool>,
    ship: Single<&Transform, With<Ship>>,
    asteroids_query: Query<(Entity, &Asteroid, &Transform)>,
    bullets_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let ship_aabb = Aabb2d::new(Vec2::ZERO, Vec2::new(40. / 2., 30. / 2.)).transformed_by(
        ship.translation.truncate(),
        ship.rotation.to_euler(EulerRot::XYZ).2,
    );

    if debug_tool.0 {
        gizmos.rect_2d(
            ship_aabb.center(),
            ship_aabb.half_size() * 2.0,
            Color::srgb(1., 0., 0.),
        );
    }

    // check ship x asteroids collision
    for (asteroid_entity, asteroid, transform) in asteroids_query {
        let asteroid_aabb = Aabb2d::new(
            Vec2::ZERO,
            Vec2::new(40. / 2., 40. / 2.) * asteroid.0 as f32,
        )
        .transformed_by(transform.translation.truncate(), 0.);

        if debug_tool.0 {
            gizmos.rect_2d(
                asteroid_aabb.center(),
                asteroid_aabb.half_size() * 2.0,
                Color::srgb(1., 0., 0.),
            );
        }

        if asteroid_aabb.intersects(&ship_aabb) {
            commands.trigger(ShipHit);
        }

        for (bullet_entity, bullet_transform) in bullets_query {
            let bullet_aabb = Aabb2d::from_point_cloud(Vec2::new(1., 1.), &[Vec2::new(0., 0.)])
                .transformed_by(bullet_transform.translation.truncate(), 0.);

            if debug_tool.0 {
                gizmos.rect_2d(bullet_aabb.center(), bullet_aabb.half_size() * 2.0, Color::srgb(0., 1., 1.));
            }

            if asteroid_aabb.contains(&bullet_aabb) {
                commands.entity(bullet_entity).despawn();
                commands.trigger(BreakAsteroid(asteroid_entity));
            }
        }
    }
}
