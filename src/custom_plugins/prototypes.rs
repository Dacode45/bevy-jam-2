use core::panic;
use std::marker::PhantomData;

use crate::systems::proto_ambient_light::proto_ambient_initialize;
use crate::systems::proto_light::{self, proto_light_initialize};
use crate::systems::starting_transform::{starting_transform_initialize, starting_transform_set};
use crate::utils::iter_hierarchy;
use crate::{loading::GltfAssets, GameState};

use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use bevy_inspector_egui::Inspectable;
use bevy_proto::prelude::ProtoData;
use bevy_proto::{ProtoComponent, Prototypical};
use serde::{Deserialize, Serialize};

pub struct CustomPrototypePlugin;

impl Plugin for CustomPrototypePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(proto_initialize)
            .add_system(proto_ambient_initialize)
            .add_system(starting_transform_initialize)
            .add_system(starting_transform_set)
            .add_system(proto_light_initialize);
    }
}

#[derive(Clone, Component, Debug, PartialEq)]
pub struct ProtoStateData<T> {
    pub name: String,
    kind: PhantomData<T>,
}

impl<T> ProtoStateData<T> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub enum ProtoState {
    NotProto,
    Initialized, // prototype name, node name
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
                for proto_name in name.split(".") {
                    if name.contains("Sun") {
                        info!("Name {:?} {:?}", name, proto_name);
                    }

                    if let Some(proto) = data.get_prototype(proto_name) {
                        info!("Found Proto {:?}", proto_name);

                        proto
                            .insert(commands.entity(entity), &data, &asset_server)
                            .insert(ProtoState::Initialized);

                        add_proto(commands.entity(entity), proto_name, name);
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

pub struct AmbientLightProto;

pub struct LightProto;

fn add_proto(mut commands: EntityCommands, proto_name: &str, node_name: &str) {
    match proto_name {
        "AmbientLight" => {
            commands.insert(ProtoStateData::<AmbientLightProto>::new(node_name));
        }
        "Light" => {
            commands.insert(ProtoStateData::<LightProto>::new(node_name));
        }
        _ => (),
    };
}
