use core::panic;

use crate::systems::proto_ambient_light::proto_ambient_initialize;
use crate::systems::proto_light::{self, proto_light_initialize};
use crate::utils::iter_hierarchy;
use crate::{loading::GltfAssets, GameState};

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use bevy_inspector_egui::Inspectable;
use bevy_proto::prelude::ProtoData;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

pub struct CustomPrototypePlugin;

impl Plugin for CustomPrototypePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(proto_initialize)
            .add_system(proto_ambient_initialize)
            .add_system(proto_light_initialize);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtoStateData {
    pub proto: String,
    pub name: String,
}

#[derive(Clone, Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub enum ProtoState {
    NotProto,
    Initialized(ProtoStateData), // prototype name, node name
}

impl ProtoState {
    fn new(proto: String, name: String) -> Self {
        Self::Initialized(ProtoStateData { proto, name })
    }

    pub fn get_name(&self) -> Option<&str> {
        match &self {
            ProtoState::Initialized(data) => Some(&data.name),
            _ => None,
        }
    }
}

impl Default for ProtoState {
    fn default() -> Self {
        Self::NotProto
    }
}

fn proto_initialize(
    mut commands: Commands,
    mut levels: Query<(Entity, &mut SceneInstance)>,
    children: Query<&Children>,
    names: Query<&Name>,
    proto_state: Query<&ProtoState>,
    data: Res<ProtoData>,
    asset_server: Res<AssetServer>,
) {
    for (lentity, _) in &mut levels {
        iter_hierarchy(lentity, &children, &mut |entity| {
            if proto_state.get(entity).is_ok() {
                return;
            }
            let mut proto_added = false;
            if let Ok(name) = names.get(entity) {
                if let Some(proto_name) = name.split(".").next() {
                    if name.contains("Sun") {
                        info!("Name {:?} {:?}", name, proto_name);
                    }
                    if let Some(proto) = data.get_prototype(proto_name) {
                        info!("Found Proto {:?}", proto_name);

                        proto
                            .insert(commands.entity(entity), &data, &asset_server)
                            .insert(ProtoState::new(proto_name.to_string(), name.to_string()));
                        proto_added = true;
                    }
                }
            }
            if !proto_added {
                commands.entity(entity).insert(ProtoState::NotProto);
            }
        });
    }
}
