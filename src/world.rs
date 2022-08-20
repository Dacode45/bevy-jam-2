use core::panic;

use crate::utils::iter_hierarchy;
use crate::{loading::GltfAssets, GameState};


use bevy::gltf::{Gltf};
use bevy::prelude::*;
use bevy::scene::SceneInstance;

use bevy_inspector_egui::Inspectable;
use bevy_proto::prelude::ProtoData;
use bevy_proto::ProtoComponent;
use serde::{Deserialize, Serialize};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Initialize).with_system(gltf_load_world),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(gltf_initialize));
    }
}

#[derive(Clone, Component, Debug, PartialEq, Reflect, Inspectable)]
#[reflect(Component)]
pub enum ProtoState {
    NotProto,
    Initialized(String),
}

impl Default for ProtoState {
    fn default() -> Self {
        Self::NotProto
    }
}
fn gltf_load_world(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    res: Res<GltfAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    let temp_level = assets_gltf.get(&res.test_level).unwrap();

    commands.spawn_bundle(SceneBundle {
        scene: temp_level.scenes[0].clone(),
        ..default()
    });

    state.set(GameState::Playing).unwrap();
}

#[derive(Clone, Serialize, Deserialize, ProtoComponent, Component)]
struct Static;

fn gltf_initialize(
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
                            .insert(ProtoState::Initialized(proto_name.to_string()));
                        proto_added = true;
                    }
                }
            }
            if !proto_added {
                commands.entity(entity).insert(ProtoState::NotProto);
            }
        });

        // level.0 = LevelState::Initialized
    }
}
