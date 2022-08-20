use bevy_flycam::PlayerPlugin;

use bevy::prelude::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin);
    }
}
