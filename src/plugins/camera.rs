use bevy::{
    camera::Hdr, post_process::bloom::Bloom, prelude::*, render::view::{DebandDither, Tonemapping}
};

use crate::{BOUNDS_RECT, resources::BloomEffect};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera);
    }
}

pub fn setup_camera(
    mut commands: Commands,
    bloom: Res<BloomEffect>
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: BOUNDS_RECT.x,
                max_height: BOUNDS_RECT.y,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Hdr,
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
    ))
    .insert_if(Bloom::default(), || bloom.0); 
}

// fn update_viewport_system(
//     window: Single<&Window>,
//     mut camera_query: Single<&mut Camera>,
// ) {
//     // if window.is_changed() {
//     //     let physical_size = UVec2::new(
//     //         window.physical_width(),
//     //         window.physical_height(),
//     //     );
//     //     if let Some(viewport) = &mut camera_query.viewport {
//     //         viewport.physical_size = physical_size;
//     //     }
//     // }
// }