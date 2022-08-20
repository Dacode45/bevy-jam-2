use crate::{loading::GltfAssets, GameState};
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(gltf_load_world));
    }
}

fn gltf_load_world(mut commands: Commands, res: Res<GltfAssets>, assets_gltf: Res<Assets<Gltf>>) {
    let temp_level = assets_gltf.get(&res.test_level).unwrap();
    commands.spawn_bundle(SceneBundle {
        scene: temp_level.scenes[0].clone(),
        ..default()
    });
}
