use crate::{
    systems::starting_transform::{starting_transform_initialize, starting_transform_set},
    world::ProtoState,
};
use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;


pub struct PrototypePlugin;

impl Plugin for PrototypePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(starting_transform_initialize)
            .add_system(starting_transform_set)
            .register_type::<ProtoState>()
            .register_inspectable::<ProtoState>();
    }
}
