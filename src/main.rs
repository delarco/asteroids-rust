use bevy::{prelude::*, window::WindowResolution};

use crate::{
    plugins::{audio_unlock, credits, debug, game, intro, menu, screenshot}, states::{AppState, InGameState, MenuState}, systems::{asteroid::on_asteroid_spawn, audio},
};

mod components;
mod events;
mod plugins;
mod resources;
mod states;
mod systems;
mod utils;
mod assets;

const BOUNDS_RECT: Vec2 = Vec2::new(1024.0, 768.0);
const BOUNDS_HALF_RECT: Vec2 = Vec2::new(BOUNDS_RECT.x / 2.0, BOUNDS_RECT.y / 2.0);
const ASTEROID_SPAWN_IGNORE_RECT: Rect = Rect::new(-BOUNDS_RECT.x / 4., -BOUNDS_RECT.y / 4., BOUNDS_RECT.x / 4., BOUNDS_RECT.y / 4.,);

const FONT_BYTES: &[u8] = include_bytes!("../assets/PressStart2P-Regular.ttf");
const FIRE_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/lasergun.wav");
const ACCELERATION_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/acceleration.wav");
const CLICK_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/click.wav");
const SHIP_HIT_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/ship_hit.wav");
const ASTEROID_HIT_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/asteroid_hit.wav");
const FIREWORK_LAUNCH_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/firework_launch.wav");
const FIREWORK_EXPLOSION_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/firework_explosion.wav");
const MOUSE_HOVER_AUDIO_BYTES: &[u8] =  include_bytes!("../assets/mouse_hover.wav");
    
fn main() {
    let mut app = App::new();

    let mut window = Window {
        title: "Asteroids".into(),
        resolution: WindowResolution::new(BOUNDS_RECT.x as u32, BOUNDS_RECT.y as u32),
        ..default()
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        window.resizable = false;
        window.present_mode = bevy::window::PresentMode::AutoNoVsync;
    }

    #[cfg(target_arch = "wasm32")]
    {
        window.canvas = Some("#bevy-canvas".to_string());
        window.fit_canvas_to_parent = true;
        window.prevent_default_event_handling = false;
    }

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }));

    let default_font = Font::from_bytes(FONT_BYTES.to_vec());

    app
        .world_mut()
        .resource_mut::<Assets<Font>>()
        .insert(AssetId::default(), default_font)
        .unwrap();

    app
        .insert_resource(resources::Volume(7))
        .insert_resource(resources::BloomEffect(true))
        .insert_resource(resources::DebugTool(false))
        .insert_resource(resources::ShipAcceleration(240.))
        .insert_resource(resources::ShipDeceleration(30.))
        .insert_resource(resources::ShipRotationSpeed(4.5))
        .insert_resource(resources::BulletCooldown(600))
        .insert_resource(resources::BulletDespawn(1300))
        .insert_resource(resources::BulletVelocity(540.))
        .insert_resource(resources::AsteroidVelocityMinMax(90., 180.))
        .insert_resource(resources::LevelNumber(0))
        .insert_resource(resources::StartingLives(3))
        .insert_resource(resources::Lives(0))
        .insert_resource(resources::BombProjectiles(30));

    app
        .init_state::<AppState>()
        .init_state::<MenuState>()
        .init_state::<InGameState>();

    app.add_observer(on_asteroid_spawn);

    // app.add_systems(PreStartup, render::setup_camera);

    app.add_systems(
        Startup,
        (
            assets::load_assets,
            audio::audio_setup,
            |mut next_app_state: ResMut<NextState<AppState>>| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    next_app_state.set(AppState::Intro);
                }

                #[cfg(target_arch = "wasm32")]
                {
                    next_app_state.set(AppState::AudioUnlock);
                }
            }
        ).chain(),
    );

    app.add_plugins((
        plugins::camera::CameraPlugin,
        debug::DebugPlugin,
        intro::intro_plugin,
        menu::menu_plugin,
        game::GamePlugin,
        credits::CreditsPlugin,
        audio_unlock::AudioUnlockPlugin,
        screenshot::ScreenshotPlugin,
    ));

    app.run();
}
