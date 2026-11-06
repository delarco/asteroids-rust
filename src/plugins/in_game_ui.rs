use bevy::prelude::*;

use crate::{BOUNDS_HALF_RECT, resources::Lives, states::AppState};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_update.run_if(in_state(AppState::InGame)));
    }
}

fn ui_update(
    mut gizmos: Gizmos,
    lives: Res<Lives>,
) {
    for n in 1..=lives.0 {
        gizmos.linestrip_2d([
            Vec2::new(-BOUNDS_HALF_RECT.x + (n as f32 * 25.) + -10., BOUNDS_HALF_RECT.y - 32. + -10.),
            Vec2::new(-BOUNDS_HALF_RECT.x + (n as f32 * 25.) + 0., BOUNDS_HALF_RECT.y - 32. + 10.),
            Vec2::new(-BOUNDS_HALF_RECT.x + (n as f32 * 25.) + 10., BOUNDS_HALF_RECT.y - 32. + -10.),
            Vec2::new(-BOUNDS_HALF_RECT.x + (n as f32 * 25.) + -10., BOUNDS_HALF_RECT.y - 32. + -10.),
        ], Color::WHITE);
    }
    
}
