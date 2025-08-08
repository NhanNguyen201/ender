use std::f32::consts::PI;

use bevy::{color::palettes::css::{BLUE, RED}, prelude::*};

use crate::{
    gameplay::{ GameState, InGameSet, Races, ShipType}, 
    plugins::AssetPack, 
    unit::{Aiming, AimingExt, Attack, AttackExt, BattleBundle, BattleStatExt, Health, HealthExt, Mobility, MobilityExt, Target}};

use avian3d::prelude::*;
pub struct UnitSpawnPlugin;
impl Plugin for UnitSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::StartupScreen), (spawn_human_mothership, spawn_alien_mothership, spawn_human_carrier, spawn_alien_carrier, spawn_auto_pilot).chain())
            .add_systems(FixedUpdate, (update_mobility_by_aiming ,update_position_by_mobility).chain().run_if(in_state(GameState::Playing)).in_set(InGameSet::EntityUpdates))
            .add_systems(Update, despawn_no_health_units.run_if(in_state(GameState::Playing)).in_set(InGameSet::DespawnEntites));
    }
}


fn spawn_human_mothership( 
    mut command: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
        let hm_mesh_component = Mesh3d(asset_pack.mesh_store.get("human_mother_ship").cloned().unwrap());
        let hm_mat_component =  MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
        }));

        command.spawn((
            hm_mesh_component.clone(),
            hm_mat_component.clone(),
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
            CollisionEventsEnabled
            

        ));

}

fn spawn_alien_mothership (
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
    let am_mesh_component = Mesh3d(asset_pack.mesh_store.get("alien_mother_ship").cloned().unwrap());
    let am_mat_compoent = MeshMaterial3d(materials.add(StandardMaterial {
        base_color: BLUE.into(),
        ..Default::default()
    }));
    commands.spawn((
    
        am_mesh_component.clone(),
        am_mat_compoent.clone(),
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
        CollisionEventsEnabled
        
        ));
}
fn spawn_human_carrier (
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
        let hc_mesh_component = Mesh3d(asset_pack.mesh_store.get("human_carrier").cloned().unwrap());
        let hc_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        }));
        for n in 0..4 {
            commands.spawn((
                // SceneRoot(human_carrier_model.clone()),
                hc_mesh_component.clone(),
                hc_mat_component.clone(),
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
                Mobility {speed: 0.0, ..default()},
                Aiming::new(Target::Position(Vec3::new(0., 0., 0.))),
                CollisionEventsEnabled

            ));
        }
}

fn spawn_alien_carrier(
    mut command: Commands, 
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

        let ac_mesh_component = Mesh3d(asset_pack.mesh_store.get("alien_carrier").cloned().unwrap());
        let ac_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..Default::default()
        }));
        for n in 0..4 {
            command.spawn((
                // SceneRoot(alien_carrier_model.clone()),

                ac_mesh_component.clone(),
                ac_mat_component.clone(),
                Transform {
                    translation: Vec3 { x: (10. * f32::sin(PI / 2. * n as f32)), y: (10. * f32::cos(PI / 2. * n as f32)), z: 20.},
                    rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                    ..default()
                },

                Name::new(format!("Alien_Carrier_{}", n)),
                Races::Scourge,
                ShipType::Carrier,
                BattleBundle::new(ShipType::Carrier),
                ColliderConstructor::TrimeshFromMesh,
                RigidBody::Dynamic,
                Mobility {speed: 0.0, ..default()},
                Aiming::new(Target::Position(Vec3::new(0., 0., 0.))),
                CollisionEventsEnabled
                

            ));
        }
}

fn spawn_auto_pilot(
    mut command: Commands, 
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
        let ha_mesh_compoent = Mesh3d(asset_pack.mesh_store.get("human_auto_pilot").cloned().unwrap());
        let ha_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            }));
        command.spawn((
            // SceneRoot(human_auto_pilot_model.clone()),
            ha_mesh_compoent.clone(),
            ha_mat_component.clone(),
            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: -20.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                ..default()
            },
            Name::new("Human_Auto_Pilot".to_string()),
            Races::Human,
            ShipType::AutoPilot,
            BattleBundle::new(ShipType::AutoPilot),
            Mobility {speed: 0.15, ..default()},
            Aiming::new(Target::Position(Vec3::new(0., 10., 0.))),
            ColliderConstructor::TrimeshFromMesh,
            CollisionEventsEnabled

        ));

        let aa_mesh_coponent = Mesh3d(asset_pack.mesh_store.get("alien_auto_pilot").cloned().unwrap());
        let aa_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
                base_color: BLUE.into(),
                ..Default::default()
        }));
        command.spawn((
            // SceneRoot(alien_auto_pilot_model.clone()),
            aa_mesh_coponent.clone(),
            aa_mat_component.clone(),
            Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 20.0},
                rotation: Quat::from_euler(EulerRot::XYZ, 0., PI, 0.),
                ..default()
            },
            Name::new("Alien_Auto_Pilot".to_string()),
            Races::Scourge,
            ShipType::AutoPilot,
            BattleBundle::new(ShipType::AutoPilot),
            Mobility {speed: 0.15, ..default()},
            Aiming::new(Target::Position(Vec3::new(0., 10., 0.))),
            ColliderConstructor::TrimeshFromMesh,
            CollisionEventsEnabled
        ));
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
    time: Res<Time>
) {
    for (mut transform, mobility) in query.iter_mut() {
        transform.translation += mobility.get_direction() * mobility.get_speed();
        let mobility_direction_to_quat = Quat::from_rotation_arc(Vec3::Z, mobility.get_direction().normalize_or_zero());
        if mobility_direction_to_quat != transform.rotation {
            // transform.rotation = Quat::from_rotation_arc(Vec3::Z, mobility.get_direction().normalize_or_zero());
            let current_rotation = transform.rotation;
            let angle = current_rotation.angle_between(mobility_direction_to_quat);
            let max_step = mobility.get_turn_speed() * time.delta_secs();
            let t = (max_step / angle).min(1.0);
            transform.rotation = current_rotation.slerp(mobility_direction_to_quat, t);
        }
    }
}

fn despawn_no_health_units(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.get_current_hit_points() <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
