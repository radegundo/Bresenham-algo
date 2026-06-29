use bevy::prelude::*;
use bevy_grid::Grid;

#[derive(Component)]
pub struct StartPoint;

#[derive(Component)]
pub struct EndPoint;

pub fn get_line(
    grid: Res<Grid>,
    start_point_query: Query<&Transform, With<StartPoint>>,
    end_point_query: Query<&Transform, With<EndPoint>>
) {
    if let Ok(p0) = start_point_query.single() && let Ok(p1) = end_point_query.single() {
        let (x0, y0) = (p0.translation.x as i32, p0.translation.y as i32);
        let (x1, y1) = (p1.translation.x as i32, p1.translation.y as i32);
        let dx = x1 - x0;
        let dy = y1 - y0;
        let p = 2 * dy - dx;
        let grid_coords = grid.get_grid_coords(Vec3::new(x0 as f32, y0 as f32, 0.0));
        println!("{}", grid_coords.unwrap());
    }
}
