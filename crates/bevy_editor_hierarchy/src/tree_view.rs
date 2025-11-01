//! Tree view UI for entity hierarchy

use bevy::prelude::*;
use bevy_editor_core::EditorEntity;
use std::collections::HashSet;

/// Resource tracking the state of the hierarchy panel UI
#[derive(Resource, Default)]
pub struct HierarchyState {
    /// Which entities are expanded (showing their children)
    pub expanded: HashSet<Entity>,
    /// Entity whose context menu is open (if any)
    pub context_menu_open: Option<Entity>,
    /// Current search filter text (empty string = no filter)
    pub search_filter: String,
    /// Last selected entity (anchor point for range selection)
    pub selection_anchor: Option<Entity>,
    /// Entity currently being dragged (for drag-and-drop reparenting)
    pub dragging: Option<Entity>,
    /// Entity that the dragged entity is currently hovering over (drop target)
    pub drop_target: Option<Entity>,
    /// Mouse position when drag started (for threshold detection)
    pub drag_start_position: Option<Vec2>,
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

/// Infer a descriptive name for an entity based on its components
fn infer_entity_name(world: &World, entity: Entity) -> String {
    let entity_ref = world.entity(entity);

    // Try to identify the entity by common component types
    if entity_ref.contains::<Camera2d>() || entity_ref.contains::<Camera3d>() {
        return format!("Camera ({})", entity.index());
    }
    if entity_ref.contains::<Window>() {
        return format!("Window ({})", entity.index());
    }
    if entity_ref.contains::<Node>() {
        return format!("UI Node ({})", entity.index());
    }
    if entity_ref.contains::<DirectionalLight>() {
        return format!("Light ({})", entity.index());
    }
    if entity_ref.contains::<Mesh3d>() {
        return format!("Mesh ({})", entity.index());
    }
    if entity_ref.contains::<Sprite>() {
        return format!("Sprite ({})", entity.index());
    }

    // Fallback to just the entity ID
    format!("Entity ({})", entity.index())
}

/// Build a flattened list of entities for rendering, respecting expand/collapse state
pub fn build_entity_tree_flat(
    world: &World,
    hierarchy_state: &HierarchyState,
    all_entities_query: &[(Entity, Option<String>)],
) -> Vec<TreeEntity> {
    let mut result = Vec::new();

    // Filter for root entities (entities without parents) and exclude editor entities
    let search_filter_lower = hierarchy_state.search_filter.to_lowercase();
    let has_search = !search_filter_lower.is_empty();

    let mut root_entities: Vec<(Entity, String)> = all_entities_query
        .iter()
        .filter(|(entity, _)| {
            // Must not have a parent AND must not be an editor entity
            if world.get::<ChildOf>(*entity).is_some() || world.get::<EditorEntity>(*entity).is_some() {
                return false;
            }

            // Filter out entities that are clearly internal/system entities
            let entity_ref = world.entity(*entity);

            // Keep entities with these "scene" components
            let is_scene_entity = entity_ref.contains::<Sprite>()
                || entity_ref.contains::<Mesh3d>()
                || entity_ref.contains::<DirectionalLight>()
                || entity_ref.contains::<Camera2d>()
                || entity_ref.contains::<Camera3d>();

            // Keep entities with a Name component (user-named entities)
            let has_name = entity_ref.contains::<Name>();

            is_scene_entity || has_name
        })
        .map(|(entity, name)| {
            let display_name = name.clone().unwrap_or_else(|| infer_entity_name(world, *entity));
            (*entity, display_name)
        })
        .filter(|(_, name)| {
            // Apply search filter if active
            !has_search || name.to_lowercase().contains(&search_filter_lower)
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
            &search_filter_lower,
            has_search,
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
    search_filter_lower: &str,
    has_search: bool,
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
                // Filter out editor entities and internal system entities from children
                .filter(|child_entity| {
                    if world.get::<EditorEntity>(*child_entity).is_some() {
                        return false;
                    }

                    // Apply same scene entity filter as roots
                    let entity_ref = world.entity(*child_entity);
                    let is_scene_entity = entity_ref.contains::<Sprite>()
                        || entity_ref.contains::<Mesh3d>()
                        || entity_ref.contains::<DirectionalLight>()
                        || entity_ref.contains::<Camera2d>()
                        || entity_ref.contains::<Camera3d>();

                    let has_name = entity_ref.contains::<Name>();

                    is_scene_entity || has_name
                })
                .map(|child_entity| {
                    let child_name = world
                        .get::<Name>(child_entity)
                        .map(|n| n.as_str().to_string())
                        .unwrap_or_else(|| infer_entity_name(world, child_entity));
                    (child_entity, child_name)
                })
                .filter(|(_, name)| {
                    // Apply search filter to children too
                    !has_search || name.to_lowercase().contains(&search_filter_lower)
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
                    search_filter_lower,
                    has_search,
                );
            }
        }
    }
}
