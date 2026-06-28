use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Resource)]
struct Grid {
    size: GridSize,
}

struct GridSize {
    x: i32,
    y: i32,
}
