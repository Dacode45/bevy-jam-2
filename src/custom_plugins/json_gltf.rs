use core::panic;

use crate::utils::iter_hierarchy;
use crate::{loading::GltfAssets, GameState};

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
use bevy_proto::prelude::ProtoData;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

pub struct JsonGltfPlugin;

use bevy_common_assets::json::JsonAssetPlugin;

impl Plugin for JsonGltfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<JsonGltf>::new(&["json"]));
    }
}

#[derive(Serialize, Deserialize)]
pub struct Node_KHR_Lights_Punctual {
    light: usize,
}

#[derive(Serialize, Deserialize)]
pub struct JsonGltfNodeEx {
    #[serde(rename = "KHR_lights_punctual")]
    khr_lights: Option<Node_KHR_Lights_Punctual>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonGltfNode {
    extensions: Option<JsonGltfNodeEx>,
    name: String,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum KHR_LightType {
    #[serde(rename = "directional")]
    Directional,
    #[serde(rename = "point")]
    Point,
}

#[derive(Serialize, Deserialize)]
pub struct KHR_LightData {
    pub color: [f32; 3],
    pub intensity: i32,
    #[serde(rename = "type")]
    pub kind: KHR_LightType,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct KHR_Lights_Punctual {
    lights: Vec<KHR_LightData>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonGltfEx {
    #[serde(rename = "KHR_lights_punctual")]
    khr_lights: KHR_Lights_Punctual,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "19bc6246-2146-40bd-931a-59beec22aec1"]
pub struct JsonGltf {
    nodes: Vec<JsonGltfNode>,
    extensions: JsonGltfEx,
}

impl JsonGltf {
    fn get_node(&self, name: &str) -> Option<&JsonGltfNode> {
        return self.nodes.iter().find(|n| n.name == name);
    }

    pub fn get_light(&self, light_index: usize) -> &KHR_LightData {
        &self.extensions.khr_lights.lights[light_index]
    }

    pub fn get_light_for_node(&self, node_name: &str) -> Option<&KHR_LightData> {
        self.get_node(node_name)
            .and_then(|node| node.extensions.as_ref())
            .and_then(|ex| ex.khr_lights.as_ref())
            .map(|node_light| self.get_light(node_light.light))
    }
}
