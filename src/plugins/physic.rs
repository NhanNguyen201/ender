use bevy::{ecs::system::SystemParam, prelude::*};
use avian3d::prelude::*;

use crate::unit::Health;


#[derive(SystemParam)]
struct CustomCollisionHooks<'w, 's> {
    interaction_health_query: Query<'w, 's, &'static Health>,
}

impl CollisionHooks for CustomCollisionHooks<'_, '_> {
    fn filter_pairs(&self, collider1: Entity, collider2: Entity, commands: &mut Commands) -> bool {
        // Example: Filter out pairs that are not of interest
        // Here we can add logic to filter based on the entities involved
        // For example, if we only want to process collisions between certain types of entities
        true // Allow all pairs for now
    }
    fn modify_contacts(&self, contacts: &mut ContactPair, commands: &mut Commands) -> bool {
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
                    aabb_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig::default(),
            );
    }
}