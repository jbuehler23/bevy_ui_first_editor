//! Infinite grid rendering using bevy_gizmos

use bevy::prelude::*;

/// Grid configuration
#[derive(Debug, Resource)]
pub struct GridConfig {
    pub enabled: bool,
    /// Size of each grid cell
    pub cell_size: f32,
    /// Number of cells to draw in each direction
    pub cell_count: i32,
    /// How many cells between major lines
    pub major_line_interval: i32,
    /// Colors
    pub color_minor: Color,
    pub color_major: Color,
    pub color_axis_x: Color,
    pub color_axis_z: Color,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cell_size: 1.0,
            cell_count: 50,
            major_line_interval: 10,
            color_minor: Color::srgba(0.2, 0.2, 0.2, 0.5),
            color_major: Color::srgba(0.4, 0.4, 0.4, 0.8),
            color_axis_x: Color::srgba(1.0, 0.3, 0.3, 0.8),
            color_axis_z: Color::srgba(0.3, 0.3, 1.0, 0.8),
        }
    }
}

/// System to draw the grid using gizmos
pub fn draw_grid(
    mut gizmos: Gizmos,
    config: Res<GridConfig>,
) {
    if !config.enabled {
        return;
    }

    let half_count = config.cell_count / 2;
    let extent = half_count as f32 * config.cell_size;

    // Draw grid lines along X axis (parallel to Z)
    for i in -half_count..=half_count {
        let offset = i as f32 * config.cell_size;
        let start = Vec3::new(-extent, 0.0, offset);
        let end = Vec3::new(extent, 0.0, offset);

        let color = if i == 0 {
            config.color_axis_z // Z axis
        } else if i % config.major_line_interval == 0 {
            config.color_major
        } else {
            config.color_minor
        };

        gizmos.line(start, end, color);
    }

    // Draw grid lines along Z axis (parallel to X)
    for i in -half_count..=half_count {
        let offset = i as f32 * config.cell_size;
        let start = Vec3::new(offset, 0.0, -extent);
        let end = Vec3::new(offset, 0.0, extent);

        let color = if i == 0 {
            config.color_axis_x // X axis
        } else if i % config.major_line_interval == 0 {
            config.color_major
        } else {
            config.color_minor
        };

        gizmos.line(start, end, color);
    }
}
