use bevy::{ecs::bundle, prelude::*};
use std::fmt;
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    StartupScreen,
    Menu,
    Playing,
    GameOverScreen
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput, 
    EntityUpdates, 
    CollisionDetection,
    DespawnEntites
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PlayerRoleHierachy {
    Commander,
    Captain,
    Pilot
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Component)]
pub enum Races {
    Human,
    Scourge
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Component)]
pub enum ShipType {
    Mothership,
    Carrier,
    AutoPilot
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub max_hit_points: f32,
    pub current_hit_points: f32,
} 

#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    pub attack_power: f32,
}


#[derive(Bundle)]
pub struct BattleBundle {
    health: Health,
    attack: Attack
}

pub trait BattleStatExt {
    fn new(ship_type: ShipType) -> Self;
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
                    attack_power: 35.0
                }
            }
        }
    }
    
}

impl fmt::Display for PlayerRoleHierachy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerRoleHierachy::Commander => write!(f, " Commander"),
            PlayerRoleHierachy::Captain => write!(f, " Captain"),
            PlayerRoleHierachy::Pilot => write!(f, " Pilot"),
        }
    }
}

#[derive(Debug,  Resource)]
pub struct PlayerRole {
    pub role: PlayerRoleHierachy
}

impl Default for PlayerRole {
    fn default() -> Self {
        Self {
            role: PlayerRoleHierachy::Commander
        }
    }
}

impl fmt::Display for PlayerRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.role)
    }
}

impl PlayerRole {
    pub fn set_role(&mut self, new_role: PlayerRoleHierachy) {
        self.role = new_role;
    }
}