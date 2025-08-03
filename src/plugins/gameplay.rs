use bevy::prelude::*;

use crate::{gameplay::{GameState, InGameSet, PlayerRole}};

pub struct GamePlayPlugin;
impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
           

            .configure_sets(
                Update, (
                    InGameSet::DespawnEntites,
                    // Defered
                    InGameSet::UserInput,
                    InGameSet::EntityUpdates,
                    InGameSet::CollisionDetection
                ).chain()
            )
            .add_systems(Update, 
                ApplyDeferred
                    .after(InGameSet::DespawnEntites)
                    .before(InGameSet::UserInput)
            )
            .add_systems(OnExit(GameState::StartupScreen), run_once);
    }
}

fn run_once(player_role: Res<PlayerRole>) {
    println!("This run onee: {:#?}", player_role.role);
}
