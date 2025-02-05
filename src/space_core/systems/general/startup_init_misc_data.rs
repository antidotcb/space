use bevy::{ecs::{system::{Commands, ResMut}}, prelude::{Transform}};


use std::{ fs, path::Path};

use crate::space_core::{bundles::ambience_sfx::{AmbienceSfxBundle}, components::{ server::Server}, resources::{gridmap_data::GridmapData, server_id::ServerId, spawn_points::{SpawnPoint, SpawnPointRaw, SpawnPoints}, world_environments::{WorldEnvironment, WorldEnvironmentRaw}}};


pub fn startup_init_misc_data(
    mut server_id : ResMut<ServerId>,
    mut map_environment : ResMut<WorldEnvironment>,
    mut gridmap_data : ResMut<GridmapData>,
    mut spawn_points_res : ResMut<SpawnPoints>,
    mut commands: Commands
) {


    let environment_json_location = Path::new("content").join("maps").join("bullseye").join("environment.json");
    let current_map_environment_raw_json : String = fs::read_to_string(environment_json_location).expect("main.rs main() Error reading map environment.json file from drive.");
    let current_map_raw_environment : WorldEnvironmentRaw = serde_json::from_str(&current_map_environment_raw_json).expect("main.rs main() Error parsing map environment.json String.");
    let current_map_environment : WorldEnvironment = WorldEnvironment::new(current_map_raw_environment);

    current_map_environment.adjust(&mut map_environment);


    let mainordered_cells_json = Path::new("content").join("maps").join("bullseye").join("mainordered.json");
    let current_map_mainordered_cells_raw_json : String = fs::read_to_string(mainordered_cells_json).expect("main.rs main() Error reading map mainordered.json drive.");
    let current_map_mainordered_cells : Vec<String> = serde_json::from_str(&current_map_mainordered_cells_raw_json).expect("main.rs main() Error parsing map mainordered.json String.");

    let details1ordered_cells_json = Path::new("content").join("maps").join("bullseye").join("details1ordered.json");
    let current_map_details1ordered_cells_raw_json : String = fs::read_to_string(details1ordered_cells_json).expect("main.rs main() Error reading map details1ordered.json drive.");
    let current_map_details1ordered_cells : Vec<String> = serde_json::from_str(&current_map_details1ordered_cells_raw_json).expect("main.rs main() Error parsing map details1ordered.json String.");

    for (i,name) in current_map_mainordered_cells.iter().rev().enumerate() {

        gridmap_data.main_name_id_map.insert(name.to_string(),i as i64);
        gridmap_data.main_id_name_map.insert(i as i64,name.to_string());

    }

    for (i,name) in current_map_details1ordered_cells.iter().rev().enumerate() {

        gridmap_data.details1_name_id_map.insert(name.to_string(), i as i64);
        gridmap_data.details1_id_name_map.insert(i as i64, name.to_string());

    }

    gridmap_data.ordered_main_names =  current_map_mainordered_cells;
    gridmap_data.ordered_details1_names = current_map_details1ordered_cells;

    


    let spawnpoints_json = Path::new("content").join("maps").join("bullseye").join("spawnpoints.json");
    let current_map_spawn_points_raw_json : String = fs::read_to_string(spawnpoints_json).expect("main.rs main() Error reading map spawnpoints.json from drive.");
    let current_map_spawn_points_raw : Vec<SpawnPointRaw> = serde_json::from_str(&current_map_spawn_points_raw_json).expect("main.rs main() Error parsing map spawnpoints.json String.");
    let mut current_map_spawn_points : Vec<SpawnPoint> = vec![];

    for raw_point in current_map_spawn_points_raw.iter() {
        current_map_spawn_points.push(SpawnPoint::new(raw_point));
    }

    spawn_points_res.list = current_map_spawn_points;
    spawn_points_res.i = 0;

    // Spawn ambience SFX
    commands.spawn().insert_bundle(AmbienceSfxBundle::new(Transform::identity()));

    // So we have one reserved Id that isnt an entity for sure
    let server_component = Server;

    server_id.id = commands.spawn().insert(server_component).id();
    

}
