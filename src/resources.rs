use bevy::prelude::*;

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u8);

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BloomEffect(pub bool);

#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct DebugTool(pub bool);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct ShipAcceleration(pub f32);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct ShipDeceleration(pub f32);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct ShipRotationSpeed(pub f32);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct BulletCooldown(pub u16);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct BulletDespawn(pub u16);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct BulletVelocity(pub f32);

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub struct AsteroidVelocityMinMax(pub f32, pub f32);

#[derive(Resource, Deref, DerefMut)]
pub struct LevelNumber(pub u8);

#[derive(Resource, Deref, DerefMut)]
pub struct StartingLives(pub u8);

#[derive(Resource, Deref, DerefMut)]
pub struct Lives(pub u8);

#[derive(Resource, Deref, DerefMut)]
pub struct BombProjectiles(pub u8);
