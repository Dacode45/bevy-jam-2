use bevy::{prelude::*};
use bevy_proto::{prelude::ProtoCommands, ProtoComponent};
use serde::{Deserialize, Serialize};



#[derive(Default, Clone, Serialize, Deserialize, Component)]
struct SunProto;

#[typetag::serde] // Required
impl ProtoComponent for SunProto {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, _asset_server: &Res<AssetServer>) {
        commands.insert_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight::default(),
            ..default()
        });
    }
}
