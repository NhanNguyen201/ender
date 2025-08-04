use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};


#[derive(Resource, Default)]
pub struct AssetPack {
    pub mesh_store: HashMap<String, Handle<Mesh>>,
    pub scene_store: HashMap<String, Handle<Scene>>,
    // pub collision_shapes: HashMap<String, Handle<CollisionShape>>,
}


pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetPack>()
            .add_systems(Startup, load_asset);
    }
}

fn load_asset(
    asset_server: Res<AssetServer>,
    mut asset_pack: ResMut<AssetPack>
) {
    let _loaded_folder: Handle<LoadedFolder>= asset_server.load_folder("models");

    let human_mother_ship_mesh: Handle<Mesh> = asset_server.load( "models/HumanMothership.glb#Mesh0/Primitive0");
    let human_mother_ship_scene: Handle<Scene> = asset_server.load("models/HumanMothership.glb#Scene0");
    println!("Loaded human_mother_ship: {:?}", human_mother_ship_mesh);
    asset_pack.scene_store.insert("human_mother_ship".to_string(), human_mother_ship_scene);
    asset_pack.mesh_store.insert("human_mother_ship".to_string(), human_mother_ship_mesh);

    let human_carrier_mesh: Handle<Mesh> = asset_server.load( "models/HumanCarrier.glb#Mesh0/Primitive0");
    let human_carrier_scene: Handle<Scene> = asset_server.load( "models/HumanCarrier.glb#Scene0");
    asset_pack.mesh_store.insert("human_carrier".to_string(), human_carrier_mesh);
    asset_pack.scene_store.insert("human_carrier".to_string(), human_carrier_scene);

    let human_auto_pilot_mesh: Handle<Mesh> = asset_server.load( "models/HumanAutoPilot.glb#Mesh0/Primitive0");
    let human_auto_pilot_scene: Handle<Scene> = asset_server.load( "models/HumanAutoPilot.glb#Scene0");
    asset_pack.mesh_store.insert("human_auto_pilot".to_string(), human_auto_pilot_mesh);
    asset_pack.scene_store.insert("human_auto_pilot".to_string(), human_auto_pilot_scene);


     let alien_mother_ship_mesh: Handle<Mesh> = asset_server.load( "models/AlienMothership.glb#Mesh0/Primitive0");
     let alien_mother_ship_scene: Handle<Scene> = asset_server.load( "models/AlienMothership.glb#Scene0");
    asset_pack.mesh_store.insert("alien_mother_ship".to_string(), alien_mother_ship_mesh);
    asset_pack.scene_store.insert("alien_mother_ship".to_string(), alien_mother_ship_scene);

    let alien_carrier_mesh: Handle<Mesh> = asset_server.load( "models/AlienCarrier.glb#Mesh0/Primitive0");
    let alien_carrier_scene: Handle<Scene> = asset_server.load( "models/AlienCarrier.glb#Scene0");
    asset_pack.mesh_store.insert("alien_carrier".to_string(), alien_carrier_mesh);
    asset_pack.scene_store.insert("alien_carrier".to_string(), alien_carrier_scene);

    let alien_auto_pilot_mesh: Handle<Mesh> = asset_server.load( "models/AlienAutoPilot.glb#Mesh0/Primitive0");
    let alien_auto_pilot_scene: Handle<Scene> = asset_server.load( "models/AlienAutoPilot.glb#Scene0");
    asset_pack.mesh_store.insert("alien_auto_pilot".to_string(), alien_auto_pilot_mesh);
    asset_pack.scene_store.insert("alien_auto_pilot".to_string(), alien_auto_pilot_scene);
   


}