use bevy::{ecs::system::SystemParam, prelude::*};
use avian3d::prelude::*;

use crate::unit::{Attack, AttackExt, Health, HealthExt};


#[derive(SystemParam)]
struct CustomCollisionHooks<'w, 's> {
    interaction_health_query: Query<'w, 's, &'static Health>,
}

impl CollisionHooks for CustomCollisionHooks<'_, '_> {
    fn filter_pairs(&self, _collider1: Entity, _collider2: Entity, _commands: &mut Commands) -> bool {
        // Example: Filter out pairs that are not of interest
        // Here we can add logic to filter based on the entities involved
        // For example, if we only want to process collisions between certain types of entities
        true // Allow all pairs for now
    }
    fn modify_contacts(&self, _contacts: &mut ContactPair, _commands: &mut Commands) -> bool {
        true
    }
}
pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default().with_collision_hooks::<CustomCollisionHooks>().set(PhysicsInterpolationPlugin::interpolate_all()))
            .insert_resource(Gravity::ZERO)
            .insert_gizmo_config(
                PhysicsGizmos {
                    aabb_color: Some(Color::LinearRgba(LinearRgba{red: 0.1,green: 0.1, blue: 0.9, alpha: 1.0})),
                    collider_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig::default(),
            )
            .add_observer(collision_observer_handle);
            
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