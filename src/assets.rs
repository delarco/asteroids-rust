use bevy::prelude::*;

use crate::{ACCELERATION_AUDIO_BYTES, ASTEROID_HIT_AUDIO_BYTES, CLICK_AUDIO_BYTES, FIRE_AUDIO_BYTES, FIREWORK_EXPLOSION_AUDIO_BYTES, FIREWORK_LAUNCH_AUDIO_BYTES, MOUSE_HOVER_AUDIO_BYTES, SHIP_HIT_AUDIO_BYTES, components::Level};

#[derive(Resource)]
pub struct ShipAssets {
    pub body: Handle<GizmoAsset>,
    pub fire: Handle<GizmoAsset>,
    pub parts: Vec<Handle<GizmoAsset>>,
}

#[derive(Resource)]
pub struct BulletAssets(pub Handle<GizmoAsset>);

#[derive(Resource)]
pub struct AsteroidAssets(pub Vec<Handle<GizmoAsset>>);

#[derive(Resource)]
pub struct LevelList(pub Vec<Level>);

#[derive(Resource)]
pub struct FireworkAssets(pub Vec<Handle<GizmoAsset>>);

#[derive(Resource)]
pub struct SoundAssets {
    pub fire: Handle<AudioSource>,
    pub acceleration: Handle<AudioSource>,
    pub click: Handle<AudioSource>,
    pub ship_hit: Handle<AudioSource>,
    pub asteroid_hit: Handle<AudioSource>,
    pub firework_launch: Handle<AudioSource>,
    pub firework_explosion: Handle<AudioSource>,
    pub mouse_hover: Handle<AudioSource>,
}

pub fn load_assets(
    mut commands: Commands,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
    asset_server: Res<AssetServer>,
) {
    // create ship assets
    let ship_vecs = [
        Vec2::new(-15., 20.),
        Vec2::new(15., 0.),
        Vec2::new(-15., -20.),
    ];

    let ship_color = Color::srgb(6.0, 6.0, 7.5);

    let mut ship_body_asset = GizmoAsset::new();
    ship_body_asset.linestrip_2d(
        ship_vecs,
        ship_color,
    );

    let mut ship_fire_asset = GizmoAsset::new();
    ship_fire_asset.arc_2d(
        Isometry2d::new(Vec2::ZERO, Rot2::radians(0.8)),
        1.5,
        20., 
        Color::srgb(3., 0., 0.)
    );
    ship_fire_asset.arc_2d(
        // Isometry2d::IDENTITY,
        Isometry2d::new(Vec2::ZERO, Rot2::radians(0.8 + 0.15)),
        1.2,
        22., 
        Color::srgb(4., 1.4, 0.)
    );
    ship_fire_asset.arc_2d(
        Isometry2d::new(Vec2::ZERO, Rot2::radians(0.8 + 0.35)),
        0.8,
        24., 
        Color::srgb(5., 5., 0.)
    );

    let ship_color = Color::srgb(7.5, 6., 6.);

    let mut ship_part_handles: Vec<Handle<GizmoAsset>> = Vec::new();
    let mut part_1 = GizmoAsset::new();
    let mut part_2 = GizmoAsset::new();
    part_1.line_2d(ship_vecs[0], ship_vecs[1], ship_color);
    part_2.line_2d(ship_vecs[1], ship_vecs[2], ship_color);
    ship_part_handles.push(gizmo_assets.add(part_1));
    ship_part_handles.push(gizmo_assets.add(part_2));

    commands.insert_resource(ShipAssets {
        body: gizmo_assets.add(ship_body_asset),
        fire: gizmo_assets.add(ship_fire_asset),
        parts: ship_part_handles,
    });

    // create bullet assets
    let mut bullet_asset = GizmoAsset::new();
    // let bullet_color = Color::srgb(7.5, 6.0, 7.5);
    let bullet_color = Color::srgb(6., 7.5, 6.);
    bullet_asset.circle_2d(Isometry2d::IDENTITY, 2., bullet_color,);
    commands.insert_resource(BulletAssets(gizmo_assets.add(bullet_asset)));

    // create asteroids assets
    let asteroids_line_vecs = vec![
        vec![
            Vec2::new(-20.0, -5.0),
            Vec2::new(-15.0, -20.0),
            Vec2::new(10.0, -20.0),
            Vec2::new(20.0, -7.0),
            Vec2::new(20.0, -3.0),
            Vec2::new(5.0, 20.0),
            Vec2::new(-5.0, 20.0),
            Vec2::new(-2.0, 2.0),
            Vec2::new(-12.0, 20.0),
            Vec2::new(-20.0, 5.0),
            Vec2::new(-10.0, 0.0),
            Vec2::new(-20.0, -5.0),
        ],
        vec![
            Vec2::new(-20.0, -10.0),
            Vec2::new(-8.0, -20.0),
            Vec2::new(0.0, -12.0),
            Vec2::new(10.0, -20.0),
            Vec2::new(20.0, -10.0),
            Vec2::new(8.0, -5.0),
            Vec2::new(20.0, 5.0),
            Vec2::new(9.0, 20.0),
            Vec2::new(-5.0, 15.0),
            Vec2::new(-10.0, 20.0),
            Vec2::new(-20.0, 10.0),
            Vec2::new(-15.0, 0.0),
            Vec2::new(-20.0, -10.0),
        ],
        vec![
            Vec2::new(-20.0, -10.0),
            Vec2::new(-3.0, -10.0),
            Vec2::new(-8.0, -20.0),
            Vec2::new(5.0, -20.0),
            Vec2::new(20.0, -10.0),
            Vec2::new(20.0, -5.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(20.0, 10.0),
            Vec2::new(11.0, 20.0),
            Vec2::new(3.0, 15.0),
            Vec2::new(-10.0, 20.0),
            Vec2::new(-20.0, 5.0),
            Vec2::new(-20.0, -10.0),
        ],
    ];

    let mut asteroids_handles: Vec<Handle<GizmoAsset>> = Vec::new();

    for asteroid in asteroids_line_vecs {
        let mut asteroid_asset = GizmoAsset::new();
        asteroid_asset.linestrip_2d(asteroid, Color::srgb(3.0, 3.0, 4.5),);
        asteroids_handles.push(gizmo_assets.add(asteroid_asset));
    }

    commands.insert_resource(AsteroidAssets(asteroids_handles));

    // create levels
    let level_list = vec![
        // Level {number: 1, asteroids_sizes: vec![1,]},
        Level {number: 1, asteroids_sizes: vec![3, 2]},
        Level {number: 2, asteroids_sizes: vec![3, 3, 2]},
        // Level {number: 3, asteroids_sizes: vec![3, 3, 3, 2]},
        // Level {number: 4, asteroids_sizes: vec![3, 3, 3, 2, 2, 1]},
    ];

    commands.insert_resource(LevelList(level_list));

    // fireworks
    let colors = vec![
        Color::srgb(4., 4., 4.),
        Color::srgb(4., 0., 0.),
        Color::srgb(4., 4., 0.),
        Color::srgb(0., 4., 0.),
        Color::srgb(0., 4., 4.),
        Color::srgb(0., 0., 4.),
    ];

    let mut firework_assets: Vec<Handle<GizmoAsset>> = vec![];

    for color in colors {
        let mut asset = GizmoAsset::new();
        asset.circle_2d(Isometry2d::IDENTITY, 1.5, color,);
        firework_assets.push(gizmo_assets.add(asset));
    }

    commands.insert_resource(FireworkAssets(firework_assets));

    // audio
    commands.insert_resource(SoundAssets {
        fire: asset_server.add(AudioSource { bytes: FIRE_AUDIO_BYTES.into()}),
        acceleration: asset_server.add(AudioSource { bytes: ACCELERATION_AUDIO_BYTES.into()}),
        click: asset_server.add(AudioSource { bytes: CLICK_AUDIO_BYTES.into()}),
        ship_hit: asset_server.add(AudioSource { bytes: SHIP_HIT_AUDIO_BYTES.into()}),
        asteroid_hit: asset_server.add(AudioSource { bytes: ASTEROID_HIT_AUDIO_BYTES.into()}),
        firework_launch: asset_server.add(AudioSource { bytes: FIREWORK_LAUNCH_AUDIO_BYTES.into()}),
        firework_explosion: asset_server.add(AudioSource { bytes: FIREWORK_EXPLOSION_AUDIO_BYTES.into()}),
        mouse_hover: asset_server.add(AudioSource { bytes: MOUSE_HOVER_AUDIO_BYTES.into()}),
    });

}
