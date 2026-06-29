use bevy::{ prelude::*, window::{ PrimaryWindow, WindowResolution } };
use bevy_grid::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My Bevy App".to_string(),
                    resolution: WindowResolution::new(500, 500),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.single() {
        let window_size = Vec2::new(window.width(), window.height());
        let mut grid = Grid::new(GridSize { x: 8, y: 8 });
        grid.build(window_size);
        commands.insert_resource(grid);

        commands.spawn((
            Camera2d,
            // Transform::from_xyz(window_size.x / 2.0, -window_size.y / 2.0, 0.0),
        ));
    }
}

fn draw_grid(grid: Res<Grid>, mut gizmos: Gizmos) {
    grid.draw(&mut gizmos);
}
