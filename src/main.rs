use bevy::{ prelude::*, window::PrimaryWindow };
use bevy_grid::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2d);

    if let Ok(window) = window_query.single() {
        let window_size = Vec2::new(window.width(), window.height());
        let mut grid = Grid::new(GridSize { x: 8, y: 8 });
        grid.build(window_size);
        commands.insert_resource(grid);
    }
}

fn draw_grid(grid: Res<Grid>, mut gizmos: Gizmos) {
    grid.draw(&mut gizmos);
}
