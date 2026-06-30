use bevy::prelude::*;
use bevy_grid::Grid;

#[derive(Component)]
pub struct StartPoint;

#[derive(Component)]
pub struct EndPoint;

pub fn get_line(
    grid: Res<Grid>,
    start_point_query: Query<&Transform, With<StartPoint>>,
    end_point_query: Query<&Transform, With<EndPoint>>,
    mut gizmos: Gizmos
) {
    if let Ok(p0) = start_point_query.single() && let Ok(p1) = end_point_query.single() {
        let (x0, y0) = (p0.translation.x as i32, p0.translation.y as i32);
        let (x1, y1) = (p1.translation.x as i32, p1.translation.y as i32);
        let start = grid
            .get_grid_coords(&Vec3::new(x0 as f32, y0 as f32, 0.0))
            .unwrap_or(Vec3::ZERO);
        let end = grid.get_grid_coords(&Vec3::new(x1 as f32, y1 as f32, 0.0)).unwrap_or(Vec3::ZERO);
        let dx = (end.x as i32) - (start.x as i32);
        let dy = (end.y as i32) - (start.y as i32);

        let (x0, y0) = (start.x as i32, start.y as i32);

        let mut y = y0;
        let mut x = x0;

        let x_step = if dx < 0 { -1 } else { 1 };
        let y_step = if dy < 0 { -1 } else { 1 };
        let dx_abs = dx.abs();
        let dy_abs = dy.abs();

        let mut line: Vec<Vec2> = vec![];

        if dx != 0 {
            if dx_abs >= dy_abs {
                // x is driving axis
                let mut p = 2 * dy_abs - dx_abs;
                for _ in 0..dx_abs + 1 {
                    line.push(Vec2::new(x as f32, y as f32));
                    if p > 0 {
                        y += y_step;
                        p += 2 * dy_abs - 2 * dx_abs;
                    } else {
                        p += 2 * dy_abs;
                    }
                    x += x_step;
                }
            } else {
                // y is driving axis
                let mut p = 2 * dx_abs - dy_abs;
                for _ in 0..dy_abs + 1 {
                    line.push(Vec2::new(x as f32, y as f32));
                    if p > 0 {
                        x += x_step;
                        p += 2 * dx_abs - 2 * dy_abs;
                    } else {
                        p += 2 * dx_abs;
                    }
                    y += y_step;
                }
            }
        } else {
            for i in 0..dy_abs + 1 {
                line.push(Vec2::new(x as f32, (y as f32) + (i as f32) * (y_step as f32)));
            }
        }
        for tile in line {
            if let Some(world_pos) = grid.get_world_coords(&tile.extend(0.0)) {
                gizmos.circle_2d(Isometry2d::from_xy(world_pos.x, world_pos.y), 10.0, Color::WHITE);
            }
        }
    }
}
