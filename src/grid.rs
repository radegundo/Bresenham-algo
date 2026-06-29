use bevy::{ prelude::*, window::PrimaryWindow };
use bevy_grid::Grid;

pub fn draw_grid(
    mut grid: ResMut<Grid>,
    mut gizmos: Gizmos,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(window) = window_query.single() {
        let window_size = Vec2::new(window.width(), window.height());
        grid.draw(&mut gizmos);
        grid.update_grid(window_size);
    }
}
