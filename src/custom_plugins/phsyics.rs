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
use bevy_proto::prelude::{ProtoCommands, ProtoData};
use bevy_proto::{ProtoComponent, Prototypical};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub struct PhsysicsPlugin;

impl Plugin for PhsysicsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct StaticCollider;

#[typetag::serde] // Required
impl ProtoComponent for StaticCollider {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands.insert(Collider::cuboid(1., 1., 1.));
        info!("static")
    }
}

#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct DynamicCollider;

#[typetag::serde] // Required
impl ProtoComponent for DynamicCollider {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(1., 1., 1.));

        info!("dynamic")
    }
}
