use bevy::{ecs::system::Command, prelude::*};
use bevy_proto::{prelude::ProtoCommands, ProtoComponent};
use serde::{Deserialize, Serialize};

use super::starting_transform::StartingTransform;

#[derive(Default, Clone, Serialize, Deserialize, Component)]
struct SunProto;

#[typetag::serde] // Required
impl ProtoComponent for SunProto {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands.insert_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight::default(),
            ..default()
        });
    }
}
