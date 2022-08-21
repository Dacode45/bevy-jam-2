use bevy::prelude::*;
use bevy_proto::{prelude::ProtoCommands, ProtoComponent};
use serde::{Deserialize, Serialize};

use crate::custom_plugins::{
    json_gltf::{JsonGltf, KHR_LightData, KHR_LightType},
    levels::CurrentLevel,
    prototypes::{LightProto, ProtoState, ProtoStateData},
};

#[derive(Default, Clone, Serialize, Deserialize, Component, ProtoComponent)]
pub struct LightComp;

pub fn proto_light_initialize(
    mut commands: Commands,
    suns: Query<Entity, With<LightComp>>,
    protos: Query<&ProtoStateData<LightProto>>,
    json_assets: Res<Assets<JsonGltf>>,
    current_level: Res<CurrentLevel>,
) {
    if let Some(level) = &current_level.level {
        if let Some(json) = json_assets.get(&level.json) {
            for entity in &suns {
                if let Ok(proto) = protos.get(entity) {
                    let name = proto.get_name();
                    info!("Going to insert");
                    let light_data = json.get_light_for_node(name);
                    if let Some(light_data) = light_data {
                        info!("Insert Light: {:?}, {:?}", name, light_data);

                        let transform = json.get_transform_for_node(name).unwrap_or(default());

                        const HALF_SIZE: f32 = 1.0;
                        // match light_data.kind {
                        //     KHR_LightType::Directional => {
                        //         commands
                        //             .entity(entity)
                        //             .insert_bundle(DirectionalLightBundle {
                        //                 directional_light: DirectionalLight {
                        //                     color: Color::from(light_data.color),
                        //                     shadow_projection: OrthographicProjection {
                        //                         left: -HALF_SIZE,
                        //                         right: HALF_SIZE,
                        //                         bottom: -HALF_SIZE,
                        //                         top: HALF_SIZE,
                        //                         near: -10.0 * HALF_SIZE,
                        //                         far: 10.0 * HALF_SIZE,
                        //                         ..default()
                        //                     },
                        //                     shadows_enabled: true,
                        //                     ..default()
                        //                 },
                        //                 transform: transform,
                        //                 ..default()
                        //             });
                        //     }
                        //     KHR_LightType::Point => {
                        //         commands.entity(entity).insert_bundle(PointLightBundle {
                        //             point_light: PointLight {
                        //                 color: Color::from(light_data.color),
                        //                 intensity: 160.0,
                        //                 shadows_enabled: true,
                        //                 ..default()
                        //             },
                        //             ..default()
                        //         });
                        //     }
                        //     _ => unreachable!("light type"),
                        // }
                    }
                }
                commands.entity(entity).remove::<LightComp>();
            }
        }
    }
}
