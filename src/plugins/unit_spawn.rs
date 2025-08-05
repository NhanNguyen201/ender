use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{gameplay::{ GameState, InGameSet, Races, ShipType}, plugins::AssetPack, unit::{Aiming, AimingExt, BattleBundle, BattleStatExt, Mobility, MobilityExt, Target}};

use avian3d::prelude::*;
pub struct UnitSpawnPlugin;
impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::StartupScreen), spawn_unit)
            .add_systems(FixedUpdate, (update_mobility_by_aiming ,update_position_by_mobility).chain().run_if(in_state(GameState::Playing)).in_set(InGameSet::EntityUpdates));
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
                Mobility {speed: 0.15, ..default()},

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
                Mobility {speed: 0.15, ..default()},

                

            ));
        }
    }
}

fn update_mobility_by_aiming(
    mut query: Query<(&mut Mobility, &Aiming, &Transform)>,
    target_transforms: Query<&Transform>,

) {
    for (mut mobility, aiming, transform) in query.iter_mut() {
        // let direction = aiming.get_direction();
        let target = aiming.get_target();
        let direction = match target {
            Some(Target::Entity(entity)) => {
                if let Ok(target_transform) = target_transforms.get(entity) {
                    (target_transform.translation - transform.translation).normalize_or_zero()
                } else {
                    Vec3::ZERO
                }
            },
            Some(Target::Position(position)) => {
                (position - transform.translation).normalize_or_zero()
            },
            None => Vec3::ZERO,
        };
        mobility.set_direction(direction);
        // mobility.set_speed(aiming.get_speed());
    }
}

fn update_position_by_mobility(
    mut query: Query<(&mut Transform, &Mobility)>,
) {
    for (mut transform, mobility) in query.iter_mut() {
        transform.translation += mobility.get_direction() * mobility.get_speed();
        transform.rotation = Quat::from_rotation_arc(Vec3::Z, mobility.get_direction().normalize_or_zero());
    }
}