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
) -> Vec<Vec<i32>> {
    vec![vec![]]
}
