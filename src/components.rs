use bevy::{ecs::component::Component, math::Vec2, time::Timer};

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct WrapAround;

#[derive(Component)]
pub struct Ship {
    pub fire_cooldown: Timer,
}

#[derive(Component)]
pub struct ShipPart;

#[derive(Component)]
pub struct ShipBody;

#[derive(Component)]
pub struct ShipFire;

#[derive(Component)]
pub struct Asteroid(pub u8);

#[derive(Component)]
pub struct Bullet {
    pub despawn_timer: Timer,
}

#[derive(Component)]
pub struct Level {
    pub number: u8,
    pub asteroids_sizes: Vec<u8>,
}
