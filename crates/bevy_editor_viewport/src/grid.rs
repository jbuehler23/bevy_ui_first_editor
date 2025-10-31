//! Infinite grid rendering using bevy_gizmos

use bevy::prelude::*;

/// Grid configuration for 2D XY plane
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
    pub color_axis_y: Color,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cell_size: 50.0,  // Larger cells for 2D pixel-based coordinates
            cell_count: 40,
            major_line_interval: 5,
            color_minor: Color::srgba(0.2, 0.2, 0.2, 0.5),
            color_major: Color::srgba(0.4, 0.4, 0.4, 0.8),
            color_axis_x: Color::srgba(1.0, 0.3, 0.3, 0.8),
            color_axis_y: Color::srgba(0.3, 1.0, 0.3, 0.8), // Green for Y axis
        }
    }
}

/// System to draw the 2D grid in XY plane using gizmos
pub fn draw_grid(
    mut gizmos: Gizmos,
    config: Res<GridConfig>,
) {
    if !config.enabled {
        return;
    }

    let half_count = config.cell_count / 2;
    let extent = half_count as f32 * config.cell_size;

    // Draw horizontal grid lines (along X axis, varying Y)
    for i in -half_count..=half_count {
        let offset = i as f32 * config.cell_size;
        let start = Vec2::new(-extent, offset);
        let end = Vec2::new(extent, offset);

        let color = if i == 0 {
            config.color_axis_x // X axis (horizontal center line)
        } else if i % config.major_line_interval == 0 {
            config.color_major
        } else {
            config.color_minor
        };

        gizmos.line_2d(start, end, color);
    }

    // Draw vertical grid lines (along Y axis, varying X)
    for i in -half_count..=half_count {
        let offset = i as f32 * config.cell_size;
        let start = Vec2::new(offset, -extent);
        let end = Vec2::new(offset, extent);

        let color = if i == 0 {
            config.color_axis_y // Y axis (vertical center line)
        } else if i % config.major_line_interval == 0 {
            config.color_major
        } else {
            config.color_minor
        };

        gizmos.line_2d(start, end, color);
    }
}
