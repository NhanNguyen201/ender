use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy_egui::EguiStartupSet;
use std::{f32::consts::FRAC_PI_2, ops::Range};

use crate::gameplay::{GameState, InGameSet};
// Define a struct for the plugin.
#[derive(Debug, Resource)]
pub struct OrbitCameraSettings {
    pub enabled: bool,
    pub active: bool,
    pub orbit_distance: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub scroll_range: Range<f32>,
    pub yaw_speed: f32, // x-axis
    pub pitch_speed: f32, // y-axis
    pub scroll_factor: f32
}

impl Default for OrbitCameraSettings {
    fn default() -> Self {
        let pitch_limit: f32 = FRAC_PI_2 - 0.01;
        let scroll_limit: f32 = 100.0;
        Self {
            enabled: true,
            active: false,
            orbit_distance: 20.0,
            pitch_range: -pitch_limit..pitch_limit,
            scroll_range: 0.5..scroll_limit,
            scroll_factor: 0.1,
            pitch_speed: 0.003,
            yaw_speed: 0.004,
        }
    }
}
pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<OrbitCameraSettings>()
             .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(PreStartup, spawn_camera.before(EguiStartupSet::InitContexts))
            .add_systems(OnExit(GameState::StartupScreen), spawn_lighs)
            .add_systems(Update, orbit_control.run_if(in_state(GameState::Playing)).in_set(InGameSet::UserInput));
          
    }
}

fn spawn_camera(mut commands: Commands) {
  
  
    // Orbit Camera
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_lighs(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::linear_rgb(0.8, 0.8, 0.8),
        brightness: 5000.0,
        ..default()
    });
    // commands.spawn((
    //    SpotLight {
    //         intensity: 10000.0,
    //         ..default()
    //    },
    //    Transform {
    //         translation: Vec3 { x: 20., y: 20., z: 10. },
    //         ..default()
    //    }
    // ));
    // commands.spawn((
    //    SpotLight {
    //         intensity: 10000.0,
    //         ..default()
    //    },
    //    Transform {
    //         translation: Vec3 { x: -20., y: 20., z: -10. },
    //         ..default()
    //    }
    // ));
    // commands.spawn((
    //    SpotLight {
    //         intensity: 5_000.0,
    //         ..default()
    //    },
    //    Transform {
    //         translation: Vec3 { x: 0., y: 10., z: 10. },
    //         ..default()
    //    }
    // ));
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: light_consts::lux::OVERCAST_DAY,
            ..default()
        },
        Transform {
            translation: Vec3 { x: 10., y: 10., z: 0. },
            ..default()
       }
        
    ));
   
}
fn orbit_control(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mut camera_settings: ResMut<OrbitCameraSettings>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) { 
    let target = Vec3::ZERO;
   
    let scroll_amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y * camera_settings.scroll_factor,
    };
    camera_settings.orbit_distance = (camera_settings.orbit_distance - scroll_amount).clamp(camera_settings.scroll_range.start, camera_settings.scroll_range.end);
    
    if mouse_buttons.just_pressed(MouseButton::Right) {
        if camera_settings.enabled == true {
            camera_settings.active = true;

        }
    }
    if mouse_buttons.just_released(MouseButton::Right) {
       camera_settings.active = false;
    }

    if camera_settings.active  { 
        let delta = mouse_motion.delta;

        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;
    
        // Conversely, we DO need to factor in delta time for mouse button inputs.
    
        // Obtain the existing pitch, yaw, and roll values from the transform.
        let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    
        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch + delta_pitch).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let yaw = yaw + delta_yaw;
        
        
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.);
    }
    

    camera.translation = target - camera.forward() * camera_settings.orbit_distance;

}

