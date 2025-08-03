use bevy::prelude::*;
use avian3d::prelude::*;

pub struct PhysicPlugin;
impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()))

            .insert_gizmo_config(
                PhysicsGizmos {
                    aabb_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig::default(),
            );
    }
}