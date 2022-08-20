use bevy::{ecs::system::Command, prelude::*};
use bevy_proto::{prelude::ProtoCommands, ProtoComponent};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct StartingTransform;

#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct StartingTransformData {
    initialized: bool,
    set_transform: bool,
    transform: Option<(Vec3, Quat, Vec3)>,
}

#[typetag::serde] // Required
impl ProtoComponent for StartingTransform {
    // Required
    fn insert_self(&self, commands: &mut ProtoCommands, asset_server: &Res<AssetServer>) {
        commands.insert(StartingTransformData::default());
    }
}

impl StartingTransformData {
    #[inline]
    pub fn set(&mut self, transform: &mut Transform) {
        if !self.initialized || self.set_transform {
            return;
        }
        if let Some(start) = &self.transform {
            transform.translation = start.0;
            transform.rotation = start.1;
            transform.scale = start.2;
            self.set_transform = true;
        }
    }

    #[inline]
    pub fn initialize(&mut self, transform: &Transform) {
        if !self.initialized {
            self.transform = Some((transform.translation, transform.rotation, transform.scale));
            self.initialized = true;
        }
    }
}

pub fn starting_transform_initialize(
    mut starting_transforms: Query<(&mut StartingTransformData, &Transform)>,
) {
    for (mut start, transform) in &mut starting_transforms {
        start.initialize(transform);
    }
}

pub fn starting_transform_set(
    mut starting_transforms: Query<(&mut StartingTransformData, &mut Transform)>,
) {
    for (mut start, mut transform) in &mut starting_transforms {
        start.set(&mut *transform);
    }
}
