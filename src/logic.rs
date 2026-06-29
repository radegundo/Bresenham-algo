use bevy::{ math::VectorSpace, prelude::* };
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
        // let p = 2 * dy - dx;
        let start = grid
            .get_grid_coords(Vec3::new(x0 as f32, y0 as f32, 0.0))
            .unwrap_or(Vec3::ZERO);
        let end = grid.get_grid_coords(Vec3::new(x0 as f32, y0 as f32, 0.0)).unwrap_or(Vec3::ZERO);
        let dx = (end.x as i32) - (start.x as i32);
        let dy = (end.y as i32) - (start.y as i32);

        let (mut x0, mut y0) = (start.x as i32, start.y as i32);
        let (mut x1, mut y1) = (end.x as i32, end.y as i32);

        let mut line = vec![];

        if dx != 0 {
            let m = dy / dx;
            let mut y = y0;

            for i in 0..dx {
                let pd = m * i + y0;
                if pd - y > y + 1 - pd {
                    line.push(Vec2::new(i as f32, (y as f32) + 1.0));
                    y = y + 1;
                } else {
                    line.push(Vec2::new(i as f32, y as f32));
                }
                println!("{}, {}", i, y);
            }
        }
    }
}
