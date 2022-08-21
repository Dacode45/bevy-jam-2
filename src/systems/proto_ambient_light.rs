use bevy::prelude::*;
use bevy_proto::{prelude::ProtoCommands, ProtoComponent};
use serde::{Deserialize, Serialize};

use crate::custom_plugins::{json_gltf::JsonGltf, levels::CurrentLevel, prototypes::ProtoState};

#[derive(Default, Clone, Serialize, Deserialize, Component, ProtoComponent)]
pub struct AmbientLightComp;

pub fn proto_ambient_initialize(
    mut commands: Commands,
    ambient: Query<(Entity, &Handle<StandardMaterial>), With<AmbientLightComp>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    for (entity, handle) in &ambient {
        let mat = materials.get(handle).unwrap();
        commands.insert_resource(AmbientLight {
            color: mat.base_color,
            brightness: 1.0 / 5.0f32,
        });
        commands.entity(entity).remove::<AmbientLightComp>();
    }
}
