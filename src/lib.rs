use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowMode};
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
mod ui;
mod gameplay;
mod plugins;
use plugins::*;

mod unit;


use crate::{gameplay::{GameState, PlayerRole}};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                 
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        position: WindowPosition::Automatic,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        title: "Bevy New Game".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .disable::<PipelinedRenderingPlugin>(),
        )
        .init_state::<GameState>()
        .init_resource::<PlayerRole>()
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PhysicPlugin)
        
        .add_plugins(UiPlugin)

        .add_plugins(ScreenModePlugin)
        .add_plugins(GamePlayPlugin)
        .add_plugins(OrbitCameraPlugin)
        .add_plugins(AxisGizmosPlugin)


        .add_plugins(UnitSpawnPlugin);
        // Add other plugins.


        // Order new `AppSystems` variants by adding them here:
     
    }
}