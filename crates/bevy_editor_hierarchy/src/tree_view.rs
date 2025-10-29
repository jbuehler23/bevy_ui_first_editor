//! Tree view UI for entity hierarchy

use bevy::prelude::*;
use bevy::utils::HashSet;

/// Resource tracking the state of the hierarchy panel UI
#[derive(Resource, Default)]
pub struct HierarchyState {
    /// Which entities are expanded (showing their children)
    pub expanded: HashSet<Entity>,
    /// Entity whose context menu is open (if any)
    pub context_menu_open: Option<Entity>,
}

/// Component marking a UI node that represents an entity in the hierarchy tree
#[derive(Component)]
pub struct EntityTreeRow {
    /// The game entity this UI row represents
    pub entity: Entity,
    /// Depth level in the hierarchy (0 = root)
    pub depth: usize,
}

/// Represents an entity and its position in the hierarchy
#[derive(Debug, Clone)]
pub struct TreeEntity {
    pub entity: Entity,
    pub name: String,
    pub depth: usize,
    pub has_children: bool,
    pub parent: Option<Entity>,
}

/// Build a flattened list of entities for rendering, respecting expand/collapse state
pub fn build_entity_tree_flat(
    world: &World,
    hierarchy_state: &HierarchyState,
) -> Vec<TreeEntity> {
    let mut result = Vec::new();

    // Find all root entities (entities without parents)
    let mut root_entities: Vec<(Entity, String)> = world
        .query_filtered::<(Entity, Option<&Name>), Without<Parent>>()
        .iter(world)
        .map(|(entity, name)| {
            let name_str = name.map(|n| n.as_str()).unwrap_or("Unnamed");
            (entity, name_str.to_string())
        })
        .collect();

    // Sort roots by name for consistent ordering
    root_entities.sort_by(|a, b| a.1.cmp(&b.1));

    // Recursively add entities to the flat list
    for (root_entity, root_name) in root_entities {
        add_entity_and_children(
            world,
            hierarchy_state,
            &mut result,
            root_entity,
            root_name,
            0,
            None,
        );
    }

    result
}

/// Recursively add an entity and its children to the flat list
fn add_entity_and_children(
    world: &World,
    hierarchy_state: &HierarchyState,
    result: &mut Vec<TreeEntity>,
    entity: Entity,
    name: String,
    depth: usize,
    parent: Option<Entity>,
) {
    // Check if this entity has children
    let children = world.get::<Children>(entity);
    let has_children = children.map(|c| !c.is_empty()).unwrap_or(false);

    // Add this entity to the list
    result.push(TreeEntity {
        entity,
        name,
        depth,
        has_children,
        parent,
    });

    // If expanded and has children, add children recursively
    if hierarchy_state.expanded.contains(&entity) {
        if let Some(children) = children {
            let mut child_data: Vec<(Entity, String)> = children
                .iter()
                .map(|&child_entity| {
                    let child_name = world
                        .get::<Name>(child_entity)
                        .map(|n| n.as_str())
                        .unwrap_or("Unnamed")
                        .to_string();
                    (child_entity, child_name)
                })
                .collect();

            // Sort children by name
            child_data.sort_by(|a, b| a.1.cmp(&b.1));

            for (child_entity, child_name) in child_data {
                add_entity_and_children(
                    world,
                    hierarchy_state,
                    result,
                    child_entity,
                    child_name,
                    depth + 1,
                    Some(entity),
                );
            }
        }
    }
}
