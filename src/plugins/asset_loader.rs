use std::collections::HashMap;

use bevy::prelude::*;

use crate::gameplay::GameState;


#[derive(Resource, Default)]
pub struct AssetPack {
    pub mesh_store: HashMap<String, Handle<Mesh>>,
    pub loading_handles: Vec<UntypedHandle>,
    // pub scene_store: HashMap<String, Handle<Scene>>,
    // pub collision_shapes: HashMap<String, Handle<CollisionShape>>,
}


pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetPack>()
            .add_systems(OnEnter(GameState::LoadingAssets), load_asset)
            .add_systems(Update, check_if_loaded.run_if(in_state(GameState::LoadingAssets)));
    }
}
fn insert_asset(
    asset_pack: &mut AssetPack,
    asset_server: &AssetServer,
    key: &str,
    mesh_path: &str,
) {
    let handle: Handle<Mesh> = asset_server.load(mesh_path);
    // let scene: Handle<Scene> = asset_server.load(scene_path);
    asset_pack.loading_handles.push(handle.clone().into());
    asset_pack.mesh_store.insert(key.to_string(), handle);
    // asset_pack.scene_store.insert(key.to_string(), scene);
}
fn check_if_loaded (
     asset_server: Res<AssetServer>,
    asset_pack: Res<AssetPack>,
    mut next_state: ResMut<NextState<GameState>>,
){
    if asset_pack
            .loading_handles
            .iter()
            .all(|handle| asset_server.is_loaded(handle))
        {
            info!("✅ All assets loaded!");
            next_state.set(GameState::StartupScreen);
        }
}
fn load_asset(
    asset_server: Res<AssetServer>,
    mut asset_pack: ResMut<AssetPack>
) {

     // Human ships
    insert_asset(&mut asset_pack, &asset_server, "human_mother_ship",
        "models/Human_mothership_mesh.glb#Mesh0/Primitive0");
    insert_asset(&mut asset_pack, &asset_server, "human_carrier",
        "models/Human_carrier_mesh.glb#Mesh0/Primitive0");
    insert_asset(&mut asset_pack, &asset_server, "human_auto_pilot",
        "models/Human_autopilot_mesh.glb#Mesh0/Primitive0");

    // Alien ships
    insert_asset(&mut asset_pack, &asset_server, "alien_mother_ship",
        "models/Alien_mothership_mesh.glb#Mesh0/Primitive0");
    insert_asset(&mut asset_pack, &asset_server, "alien_carrier",
        "models/Alien_carrier_mesh.glb#Mesh0/Primitive0");
    insert_asset(&mut asset_pack, &asset_server, "alien_auto_pilot",
        "models/Alien_autopilot_mesh.glb#Mesh0/Primitive0");
   


}