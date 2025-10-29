//! Entity operations (create, delete, reparent, etc.)

use bevy::prelude::*;

/// Create a new empty entity in the scene
pub fn create_empty_entity(world: &mut World) -> Entity {
    world.spawn(Transform::default()).id()
}

/// Delete an entity and optionally its children
pub fn delete_entity(world: &mut World, entity: Entity, recursive: bool) {
    if recursive {
        // Delete children recursively
        // TODO: Implement
    }
    world.despawn(entity);
}

/// Reparent an entity to a new parent
pub fn reparent_entity(world: &mut World, entity: Entity, new_parent: Option<Entity>) {
    // TODO: Implement using Parent/Children components
}
