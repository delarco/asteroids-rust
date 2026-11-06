use bevy::{ecs::{query::With, system::Query}, transform::components::Transform};

use crate::{BOUNDS_HALF_RECT, components::WrapAround};

/// Wrap objects into screen space. Any object levaving the screen
/// gets "teleported" to the oposite side.
pub fn wrap_around_system(mut query: Query<&mut Transform, With<WrapAround>>) {
    for mut transform in &mut query {
        // Wrap X axis
        if transform.translation.x > BOUNDS_HALF_RECT.x {
            transform.translation.x = -BOUNDS_HALF_RECT.x;
        } else if transform.translation.x < -BOUNDS_HALF_RECT.x {
            transform.translation.x = BOUNDS_HALF_RECT.x;
        }

        // Wrap Y axis
        if transform.translation.y > BOUNDS_HALF_RECT.y {
            transform.translation.y = -BOUNDS_HALF_RECT.y;
        } else if transform.translation.y < -BOUNDS_HALF_RECT.y {
            transform.translation.y = BOUNDS_HALF_RECT.y;
        }
    }
}
