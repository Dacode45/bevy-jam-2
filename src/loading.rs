use crate::{custom_plugins::json_gltf::JsonGltf, GameState};
use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<GltfAssets>()
                .with_collection::<JsonAssets>()
                .continue_to_state(GameState::Initialize),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct JsonAssets {
    #[asset(path = "gltf/test_level.json")]
    pub test_level: Handle<JsonGltf>,
}

#[derive(AssetCollection)]
pub struct GltfAssets {
    #[asset(path = "gltf/test_level.gltf")]
    pub test_level: Handle<Gltf>,
}
