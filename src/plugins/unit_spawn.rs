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
            .add_systems(FixedUpdate, despawn_no_health_units.run_if(in_state(GameState::Playing)).in_set(InGameSet::DespawnEntites));
    }
}


fn spawn_human_mothership( 
    mut command: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
        if let Some(hm_mesh_component) = asset_pack.mesh_store.get("human_mother_ship").cloned() {
            let hm_mat_component =  MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            }));
    
            command.spawn((
                Mesh3d(hm_mesh_component.clone()),
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

        } else {
            error!("Human Mothership mesh not found in asset pack.");
        }

}

fn spawn_alien_mothership (
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
    if let Some(am_mesh_component) = asset_pack.mesh_store.get("alien_mother_ship").cloned() {
        let am_mat_compoent = MeshMaterial3d(materials.add(StandardMaterial {
            base_color: BLUE.into(),
            ..Default::default()
        }));
        commands.spawn((
        
            Mesh3d(am_mesh_component.clone()),
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

    } else {
        error!("Alien Mothership mesh not found in asset pack.");
    }
}
fn spawn_human_carrier (
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
        if let Some (hc_mesh_component) = asset_pack.mesh_store.get("human_carrier").cloned() {
            let hc_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::LinearRgba(LinearRgba::new(0.8, 0.1, 0.1, 1.)).into(),
                ..Default::default()
            }));
            for n in 0..4 {
                commands.spawn((
                    Mesh3d(hc_mesh_component.clone()),
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

        } else {
            error!("Human Carrier mesh not found in asset pack.");
        }
}

fn spawn_alien_carrier(
    mut command: Commands, 
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(ac_mesh_component) = asset_pack.mesh_store.get("alien_carrier").cloned() {

        let ac_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::new(0.1, 0.1, 0.8, 1.)).into(),
            ..Default::default()
        }));
        for n in 0..4 {
            command.spawn((
    
                Mesh3d(ac_mesh_component.clone()),
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
    } else {
        error!("Alien Carrier mesh not found in asset pack.");
    }
}

fn spawn_auto_pilot(
    mut command: Commands, 
    asset_pack: Res<AssetPack>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    if let Some(ha_mesh_compoent) = asset_pack.mesh_store.get("human_auto_pilot").cloned() {

        let ha_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::LinearRgba(LinearRgba::new(0.8, 0.2, 0.1, 1.)).into(),

            ..Default::default()
        }));
        command.spawn((
            Mesh3d(ha_mesh_compoent.clone()),
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
    
       
    } else {
        error!("Human Auto Pilot mesh not found in asset pack.");
    };

    if let Some(aa_mesh_coponent) = asset_pack.mesh_store.get("alien_auto_pilot").cloned() {
        let aa_mat_component = MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::LinearRgba(LinearRgba::new(0.1, 0.2, 0.8, 1.)).into(),

                ..Default::default()
        }));
        command.spawn((
            Mesh3d(aa_mesh_coponent.clone()),
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

    } else {
        error!("Alien Auto Pilot mesh not found in asset pack.");
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
