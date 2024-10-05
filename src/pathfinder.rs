use crate::vector::Vector2;

pub trait Pathfinder {
    fn init(
        &mut self,
        rows: i32,
        cols: i32,
        start_pos: Vector2<usize>,
        end_pos: Vector2<usize>,
        walls: Vec<Vec<bool>>,
    );

    fn step(&mut self) -> Option<Vector2<usize>>;

    fn get_frontier(&self) -> Vec<Vector2<usize>>;

    fn get_visited(&self) -> Vec<Vec<bool>>;

    fn deinit(&mut self);
}
