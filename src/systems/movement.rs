use bevy::prelude::*;

use crate::components::Velocity;

pub fn movement_system(entities: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in entities {
        transform.translation += velocity.0.extend(0.) * time.delta_secs();
    }
}
