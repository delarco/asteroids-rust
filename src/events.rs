use bevy::{ecs::{entity::Entity, event::Event}, math::Vec2};

use crate::systems::bullet::Projectile;

#[derive(Event)]
pub struct FireProjectile(pub Projectile);

#[derive(Event)]
pub struct BreakAsteroid(pub Entity);

#[derive(Event)]
pub struct ShipHit;

#[derive(Event)]
pub struct BreakShip;

#[derive(Event)]
pub struct SpawnAsteroid{
    pub size: u8,
    pub from_point: Option<Vec2>,
    pub ignore_ship_area: bool,
}

#[derive(Event)]
pub struct NextLevel;
