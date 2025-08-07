use std::f32::consts::PI;

use bevy::prelude::*;

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
            .add_systems(Update, despawn_no_health_units.run_if(in_state(GameState::Playing)).in_set(InGameSet::DespawnEntites))
            .add_observer(collision_observer_handle);
    }
}


fn spawn_human_mothership( 
    mut command: Commands,
    asset_pack: Res<AssetPack>
){
      if let Some(human_mother_ship_model) = asset_pack.scene_store.get("human_mother_ship") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("human_mother_ship").cloned().unwrap());
        command.spawn((
            SceneRoot(human_mother_ship_model.clone()),
            mesh_component,
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
    };

}

fn spawn_alien_mothership (
    mut commands: Commands,
    asset_pack: Res<AssetPack>
){
    if let Some(alien_mother_ship_model) = asset_pack.scene_store.get("alien_mother_ship") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("alien_mother_ship").cloned().unwrap());
        commands.spawn((
        
            SceneRoot(alien_mother_ship_model.clone()),
            mesh_component,

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
    };
}
fn spawn_human_carrier (
    mut commands: Commands,
    asset_pack: Res<AssetPack>
){
    if let Some(human_carrier_model) = asset_pack.scene_store.get("human_carrier") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("human_carrier").cloned().unwrap());
        for n in 0..4 {
            commands.spawn((
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
                Mobility {speed: 0.0, ..default()},
                Aiming::new(Target::Position(Vec3::new(0., 0., 0.))),
                CollisionEventsEnabled

            ));
        }
    };
}

fn spawn_alien_carrier(mut command: Commands, asset_pack: Res<AssetPack>) {
    if let Some(alien_carrier_model) = asset_pack.scene_store.get("alien_carrier") {
        let mesh_component = Mesh3d(asset_pack.mesh_store.get("alien_carrier").cloned().unwrap());
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
}

fn spawn_auto_pilot(mut command: Commands, asset_pack: Res<AssetPack>) {
    if let Some(human_auto_pilot_model) = asset_pack.scene_store.get("human_auto_pilot") {
        let mesh_compoent = Mesh3d(asset_pack.mesh_store.get("human_auto_pilot").cloned().unwrap());
        command.spawn((
            SceneRoot(human_auto_pilot_model.clone()),
            mesh_compoent,
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
    };

    if let Some(alien_auto_pilot_model) = asset_pack.scene_store.get("alien_auto_pilot") {
        let mesh_coponent = Mesh3d(asset_pack.mesh_store.get("alien_auto_pilot").cloned().unwrap());
        command.spawn((
            SceneRoot(alien_auto_pilot_model.clone()),
            mesh_coponent,
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

fn collision_observer_handle(
    trigger: Trigger<OnCollisionStart>,
    mut health_query: Query<&mut Health>,
    attack_query: Query<&Attack>,
) {
    let first_ent = trigger.target();
    let other_entity = trigger.collider;

    if let (Ok(mut first_health), Ok(other_attack)) = (health_query.get_mut(first_ent), attack_query.get(other_entity)) {
        first_health.take_damage(other_attack.get_attack_power());
    }
    // if let (Ok(mut other_health), Ok(first_attack)) = (health_query.get_mut(other_entity), attack_query.get(first_ent)) {
    //     other_health.take_damage(first_attack.get_attack_power());
    // }
}