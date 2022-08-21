#[macro_use]
extern crate bevy_discovery;

mod actions;
mod audio;
mod custom_plugins;
mod loading;
mod menu;
mod player;
// mod prototypes;
mod systems;
mod utils;
// mod world;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use crate::custom_plugins::discovery::DiscoveryPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_proto::ProtoPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use custom_plugins::levels::LevelPlugin;
use custom_plugins::prototypes::CustomPrototypePlugin;
use custom_plugins::CustomPlugins;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // Initialize post loading
    Initialize,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(DiscoveryPlugin)
            .add_plugin(CustomPlugins)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(CustomPrototypePlugin)
            .add_plugin(ProtoPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                // .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(OverlayPlugin {
                    font_size: 32.0,
                    ..default()
                })
                .add_plugin(DebugLinesPlugin::default())
                .add_plugin(WorldInspectorPlugin::new());
        }
    }
}
