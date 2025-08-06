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
fn insert_asset(
    asset_pack: &mut AssetPack,
    asset_server: &AssetServer,
    key: &str,
    mesh_path: &str,
    scene_path: &str,
) {
    let mesh: Handle<Mesh> = asset_server.load(mesh_path);
    let scene: Handle<Scene> = asset_server.load(scene_path);
    asset_pack.mesh_store.insert(key.to_string(), mesh);
    asset_pack.scene_store.insert(key.to_string(), scene);
}

fn load_asset(
    asset_server: Res<AssetServer>,
    mut asset_pack: ResMut<AssetPack>
) {
    let _loaded_folder: Handle<LoadedFolder>= asset_server.load_folder("models");

     // Human ships
    insert_asset(&mut asset_pack, &asset_server, "human_mother_ship",
        "models/HumanMothership_mesh.glb#Mesh0/Primitive0", "models/HumanMothership.glb#Scene0");
    insert_asset(&mut asset_pack, &asset_server, "human_carrier",
        "models/HumanCarrier_mesh.glb#Mesh0/Primitive0", "models/HumanCarrier.glb#Scene0");
    insert_asset(&mut asset_pack, &asset_server, "human_auto_pilot",
        "models/HumanAutoPilot_mesh.glb#Mesh0/Primitive0", "models/HumanAutoPilot.glb#Scene0");

    // Alien ships
    insert_asset(&mut asset_pack, &asset_server, "alien_mother_ship",
        "models/AlienMothership_mesh.glb#Mesh0/Primitive0", "models/AlienMothership.glb#Scene0");
    insert_asset(&mut asset_pack, &asset_server, "alien_carrier",
        "models/AlienCarrier_mesh.glb#Mesh0/Primitive0", "models/AlienCarrier.glb#Scene0");
    insert_asset(&mut asset_pack, &asset_server, "alien_auto_pilot",
        "models/AlienAutoPilot_mesh.glb#Mesh0/Primitive0", "models/AlienAutoPilot.glb#Scene0");
   


}