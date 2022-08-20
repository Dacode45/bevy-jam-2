use bevy::{
    ecs::{archetype::Archetypes, component::ComponentId},
    prelude::*,
};

pub fn iter_hierarchy(
    entity: Entity,
    children_query: &Query<&Children>,
    f: &mut impl FnMut(Entity),
) {
    (f)(entity);
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter().copied() {
            iter_hierarchy(child, children_query, f);
        }
    }
}

pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().contains(entity) {
            return Some(archetype.components());
        }
    }
    None
}
