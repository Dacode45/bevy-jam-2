use self::json_gltf::JsonGltfPlugin;

pub mod discovery;
pub mod json_gltf;
pub mod levels;
pub mod phsyics;
pub mod prototypes;

use crate::utils::iter_hierarchy;
use crate::{loading::GltfAssets, GameState};

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use bevy_inspector_egui::Inspectable;
use bevy_proto::prelude::ProtoData;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

pub struct CustomPlugins;

impl Plugin for CustomPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonGltfPlugin);
    }
}
