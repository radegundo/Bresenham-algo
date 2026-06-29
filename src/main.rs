use bevy::{ gltf::gltf_ext::material, prelude::*, window::{ PrimaryWindow, WindowResolution } };
use bevy_grid::prelude::*;

use crate::logic::*;

mod grid;
mod logic;

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
        .add_systems(Update, grid::draw_grid)
        .add_systems(Update, input_system)
        .add_systems(Update, get_line)
        .run();
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(window) = window_query.single() {
        //Initialize grid and add it to resources
        let window_size = Vec2::new(window.width(), window.height());
        let mut grid = Grid::new(GridSize { x: 8, y: 8 });
        grid.build(window_size);
        commands.insert_resource(grid);

        //Spawn Camera
        commands.spawn(Camera2d);
    }
    //Spawn Start Point
    commands.spawn((
        Transform::from_xyz(-100.0, -100.0, 0.0),
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        StartPoint,
    ));
    //Spawn End Point
    commands.spawn((
        Transform::from_xyz(100.0, 100.0, 0.0),
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        EndPoint,
    ));
}

//Input to move start and end points
fn input_system(
    mut queries: ParamSet<
        (Query<&mut Transform, With<StartPoint>>, Query<&mut Transform, With<EndPoint>>)
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if let Ok(mut start) = queries.p0().single_mut() {
        let mut dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyD) {
            dir.x += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            dir.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            dir.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            dir.y -= 10.0;
        }
        start.translation += dir;
    } else {
        return;
    }

    if let Ok(mut end) = queries.p1().single_mut() {
        let mut dir = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            dir.x += 10.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            dir.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            dir.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            dir.y -= 10.0;
        }
        end.translation += dir;
    } else {
        return;
    }
}
