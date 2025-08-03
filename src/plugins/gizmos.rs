use bevy::prelude::*;
use std::f32::consts::PI;
#[derive(Default, Reflect, GizmoConfigGroup)]
struct GroundGizmos;


pub struct AxisGizmosPlugin;
impl Plugin for AxisGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<GroundGizmos>()
            .add_systems(Update, draw_example_collection);
    }
}

fn draw_example_collection(
    mut my_gizmos: Gizmos<GroundGizmos>,
) {
    my_gizmos.grid(
        Quat::from_rotation_x(PI / 2.),
        UVec2::splat(20),
        Vec2::new(2., 2.),
        // Light gray
        LinearRgba {red: 0.1, blue: 0.1, green: 0.1, alpha: 0.5},
    );
}