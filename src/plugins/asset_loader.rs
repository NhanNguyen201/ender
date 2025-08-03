use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};


#[derive(Resource, Default)]
pub struct AssetPack {
    pub scene_store: HashMap<String, Handle<Mesh>>,
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

    let human_mother_ship: Handle<Mesh> = asset_server.load( "models/HumanMothership.glb#Mesh0/Primitive0");
    println!("Loaded human_mother_ship: {:?}", human_mother_ship);
    asset_pack.scene_store.insert("human_mother_ship".to_string(), human_mother_ship);

    let human_carrier: Handle<Mesh> = asset_server.load( "models/HumanCarrier.glb#Mesh0/Primitive0");
    asset_pack.scene_store.insert("human_carrier".to_string(), human_carrier);

    let human_auto_pilot: Handle<Mesh> = asset_server.load( "models/HumanAutoPilot.glb#Mesh0/Primitive0");
    asset_pack.scene_store.insert("human_auto_pilot".to_string(), human_auto_pilot);


     let alien_mother_ship: Handle<Mesh> = asset_server.load( "models/AlienMothership.glb#Mesh0/Primitive0");
    asset_pack.scene_store.insert("alien_mother_ship".to_string(), alien_mother_ship);

    let alien_carrier: Handle<Mesh> = asset_server.load( "models/AlienCarrier.glb#Mesh0/Primitive0");
    asset_pack.scene_store.insert("alien_carrier".to_string(), alien_carrier);

    let alien_auto_pilot: Handle<Mesh> = asset_server.load( "models/AlienAutoPilot.glb#Mesh0/Primitive0");
    asset_pack.scene_store.insert("alien_auto_pilot".to_string(), alien_auto_pilot);
   


}