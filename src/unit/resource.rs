use bevy::prelude::*;

use crate::gameplay::ShipType;

#[derive( Debug, Clone, Copy)]
pub enum Target {
    Entity(Entity),
    Position(Vec3),
}

pub trait AimingExt {
    fn new(target: Target) -> Self;
    fn get_target(&self) -> Option<Target>;
    fn set_target(&mut self, target: Target);
}

pub trait HealthExt {
    fn get_max_hit_points(&self) -> f32;
    fn get_current_hit_points(&self) -> f32;
    fn take_damage(&mut self, damage: f32);
}

pub trait AttackExt {
    fn get_attack_power(&self) -> f32;
}
pub trait BattleStatExt {
    fn new(ship_type: ShipType) -> Self;
}

pub trait MobilityExt {
    fn new(speed: f32, turn_speed: f32,  direction: Vec3) -> Self;
    fn get_speed(&self) -> f32;
    fn get_turn_speed(&self) -> f32;
    fn get_direction(&self) -> Vec3;
    fn set_speed(&mut self, speed: f32);
    fn set_direction(&mut self, direction: Vec3);
}