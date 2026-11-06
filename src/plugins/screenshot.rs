use std::{path::Path, time::{SystemTime, UNIX_EPOCH}};

use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
};

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, screenshot_check);
    }
}

fn screenshot_check(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {

        let directory = Path::new("./screenshots");

        if !directory.exists() {
            std::fs::create_dir(directory).unwrap();
        }

        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let filepath = Path::new(directory)
            .join(format!("screenshot-{}.png", since_epoch.as_millis()));

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(filepath));
    }
}
