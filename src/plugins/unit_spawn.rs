use std::f32::consts::PI;

use bevy::{prelude::*, tasks::futures_lite::future::Race};

use crate::{gameplay::{BattleBundle, BattleStatExt, GameState, Races, ShipType}, plugins::{physic, AssetPack}};

use avian3d::prelude::*;
pub struct UnitSpawnPlugin;
impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::StartupScreen), spawn_unit);
    }
}

fn spawn_unit(mut command: Commands, asset_pack: Res<AssetPack>) {
    if let Some(human_mother_ship_model) = asset_pack.scene_store.get("human_mother_ship") {
        command.spawn((
            Mesh3d(human_mother_ship_model.clone()),
            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: -30.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                ..default()
            },
            Name::new("Human Mothership"),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Dynamic,
            Races::Human,
            ShipType::Mothership,
            BattleBundle::new(ShipType::Mothership),
        ));
    };


    if let Some(alien_mother_ship_model) = asset_pack.scene_store.get("alien_mother_ship") {
        command.spawn((
            Mesh3d(alien_mother_ship_model.clone()),
            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 30.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                ..default()
            },
            Name::new("Alien Mothership"),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Dynamic,
            Races::Scourge,
            ShipType::Mothership,
            BattleBundle::new(ShipType::Mothership),

        ));
    };
}