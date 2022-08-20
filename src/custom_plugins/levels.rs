use crate::{
    custom_plugins::json_gltf::JsonGltf,
    loading::{GltfAssets, JsonAssets},
    GameState,
};

use bevy::{ecs::system::Command, gltf::Gltf, prelude::*, utils::HashMap};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentLevelName(LevelName::None))
            .insert_resource(CurrentLevel::default())
            .add_system_set(
                SystemSet::on_update(GameState::Initialize).with_system(levels_initialize),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(levels_startup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(level_spawn));
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LevelName {
    None, // Initial
    TestLevel,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub json: Handle<JsonGltf>,
    pub gltf: Handle<Gltf>,
}

pub type Levels = HashMap<LevelName, Level>;

#[derive(Debug)]
pub struct CurrentLevelName(LevelName);

#[derive(Debug, Default)]
pub struct CurrentLevel {
    pub level: Option<Level>,
    pub root: Option<Entity>,
}

// Stuff common to the systems
pub fn levels_startup(mut commands: Commands, current_level_name: Res<CurrentLevelName>) {
    if current_level_name.0 == LevelName::None {
        info!("Spawning new level");
        commands.set_level(LevelName::TestLevel);
    }
}

// Loading the handles
fn levels_initialize(
    mut commands: Commands,
    json: Res<JsonAssets>,
    gltf: Res<GltfAssets>,
    mut state: ResMut<State<GameState>>,
) {
    info!("Running level init");
    let mut levels: Levels = HashMap::new();

    levels.insert(
        LevelName::TestLevel,
        Level {
            json: json.test_level.clone(),
            gltf: gltf.test_level.clone(),
        },
    );

    commands.insert_resource(levels);
    commands.insert_resource(CurrentLevelName(LevelName::None));
    state.set(GameState::Playing).unwrap();
}

pub fn level_spawn(
    mut commands: Commands,
    current_level_name: Res<CurrentLevelName>,
    mut current_level: ResMut<CurrentLevel>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    if current_level_name.is_changed() {
        info!("Loaded {:?}", assets_gltf.iter().count());
        info!("Current Level Name: {:?}", current_level_name);
        if let Some(level) = &mut current_level.level {
            info!("Current Level: {:?}", level);
            let level_gltf = assets_gltf
                .get(&level.gltf)
                .expect("No level gltf available");
            current_level.root = Some(
                commands
                    .spawn_bundle(SceneBundle {
                        scene: level_gltf.scenes[0].clone(),
                        ..default()
                    })
                    .id(),
            );
        }
    }
}

struct SetLevel {
    level_name: LevelName,
}

// create two entities, the second one referring to the first one
impl Command for SetLevel {
    fn write(self: Self, world: &mut World) {
        let next_level = world.resource::<Levels>().get(&self.level_name).cloned();

        let mut level = world.resource_mut::<CurrentLevel>();
        level.level = next_level;

        let entity = level.root.clone();

        if let Some(entity) = entity {
            world.despawn(entity);
        }

        world.insert_resource(CurrentLevelName(self.level_name));
    }
}

trait LevelCommandsExt {
    fn set_level(&mut self, level_name: LevelName) -> &mut Self;
}

impl<'a, 'b> LevelCommandsExt for Commands<'a, 'b> {
    fn set_level(&mut self, level_name: LevelName) -> &mut Self {
        self.add(SetLevel { level_name });
        self
    }
}
