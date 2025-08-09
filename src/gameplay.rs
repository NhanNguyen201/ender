use bevy::prelude::*;
use std::fmt;

pub mod resource;
pub use resource::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    LoadingAssets,
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