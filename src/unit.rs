use bevy::prelude::*;
pub mod resource;
pub use resource::*;

use crate::gameplay::ShipType;

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub max_hit_points: f32,
    pub current_hit_points: f32,
} 

impl HealthExt for Health {
    fn get_max_hit_points(&self) -> f32 {
        self.max_hit_points
    }

    fn get_current_hit_points(&self) -> f32 {
        self.current_hit_points
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    pub attack_power: f32,
}

impl AttackExt for Attack {
    fn get_attack_power(&self) -> f32 {
        self.attack_power
    }
    
}
#[derive(Component, Debug, Clone, Copy)]
pub struct Mobility {
    pub speed: f32,
    pub direction: Vec3,
}

impl Default for Mobility {
    fn default() -> Self {
        Mobility {
            speed: 0.0,
            direction: Vec3::ZERO,
        }
    }
}

impl MobilityExt for Mobility {
    fn new(speed: f32, direction: Vec3) -> Self {
        Mobility { speed, direction }
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_direction(&self) -> Vec3 {
        self.direction
    }

    fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction;
    }
}

#[derive(Bundle)]
pub struct BattleBundle {
    health: Health,
    attack: Attack
}



impl BattleStatExt for BattleBundle {
    fn new(ship_type: ShipType) -> Self {
        match ship_type {
            ShipType::Mothership => BattleBundle {
                health: Health {
                    max_hit_points: 10000.0,
                    current_hit_points: 10000.0
                },
                attack: Attack {
                    attack_power: 2000.0
                }
            },
            ShipType::Carrier => BattleBundle {
                 health: Health {
                    max_hit_points: 2000.0,
                    current_hit_points: 2000.0
                },
                attack: Attack {
                    attack_power: 200.0
                }
            },
            ShipType::AutoPilot => BattleBundle {
                 health: Health {
                    max_hit_points: 200.0,
                    current_hit_points: 200.0
                },
                attack: Attack {
                    attack_power: 25.0
                }
            }
        }
    }
    
}