use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{gameplay::{ GameState, InGameSet, Races, ShipType}, plugins::AssetPack, unit::{BattleBundle, BattleStatExt, Mobility, MobilityExt}};

use avian3d::prelude::*;
pub struct UnitSpawnPlugin;
impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::StartupScreen), spawn_unit)
            .add_systems(Update, update_position_by_mobility.run_if(in_state(GameState::Playing)).in_set(InGameSet::EntityUpdates));
    }
}

fn spawn_unit(mut command: Commands, asset_pack: Res<AssetPack>) {
    if let Some(human_mother_ship_model) = asset_pack.scene_store.get("human_mother_ship") {
        command.spawn((
            SceneRoot(human_mother_ship_model.clone()),
            Mesh3d(asset_pack.mesh_store.get("human_mother_ship").cloned().unwrap_or_default()),
            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: -30.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                ..default()
            },
            Name::new("Human_Mothership".to_string()),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Dynamic,
            Races::Human,
            ShipType::Mothership,
            BattleBundle::new(ShipType::Mothership),
            Mobility::new(0.0, Vec3::ZERO),

        ));
    };


    if let Some(alien_mother_ship_model) = asset_pack.scene_store.get("alien_mother_ship") {
        command.spawn((
            SceneRoot(alien_mother_ship_model.clone()),
            Mesh3d(asset_pack.mesh_store.get("alien_mother_ship").cloned().unwrap_or_default()),

            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 30.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                ..default()
            },
            Name::new("Alien_Mothership".to_string()),
            ColliderConstructor::TrimeshFromMesh,
            RigidBody::Dynamic,
            Races::Scourge,
            ShipType::Mothership,
            BattleBundle::new(ShipType::Mothership),
            Mobility::new(0.0, Vec3::ZERO),

        ));
    };

    if let Some(human_carrier_model) = asset_pack.scene_store.get("human_carrier") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("human_carrier").cloned().unwrap_or_default());
        for n in 0..4 {
            command.spawn((
                SceneRoot(human_carrier_model.clone()),
                mesh_component.clone(),
                Transform {
                    translation: Vec3 { x: (10. * f32::sin( PI / 2. * n as f32)), y: (10. * f32::cos( PI / 2. * n as f32)), z: -20.},
                    rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                    ..default()
                },
                Name::new(format!("Human_Carrier_{}", n)),
                Races::Human,
                ShipType::Carrier,
                BattleBundle::new(ShipType::Carrier),
                ColliderConstructor::TrimeshFromMesh,
                RigidBody::Dynamic,
                Mobility::new(0.5, Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
            ));
        }
    };

    if let Some(alien_carrier_model) = asset_pack.scene_store.get("alien_carrier") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("alien_carrier").cloned().unwrap_or_default());
        for n in 0..4 {
            command.spawn((
                SceneRoot(alien_carrier_model.clone()),
                mesh_component.clone(),
                Transform {
                    translation: Vec3 { x: (10. * f32::sin(PI / 2. * n as f32)), y: (10. * f32::cos(PI / 2. * n as f32)), z: 20.},
                    rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                    ..default()
                },
                Name::new(format!("Alien_Carrier_{}", n)),
                Races::Human,
                ShipType::Carrier,
                BattleBundle::new(ShipType::Carrier),
                ColliderConstructor::TrimeshFromMesh,
                RigidBody::Dynamic,
                Mobility::new(0.5, Vec3 { x: 0.0, y: 0.0, z: 1.0 }),

            ));
        }
    }
}

fn update_position_by_mobility(
    mut query: Query<(&mut Transform, &Mobility)>,
) {
    for (mut transform, mobility) in query.iter_mut() {
        transform.translation += mobility.get_direction() * mobility.get_speed();
    }
}