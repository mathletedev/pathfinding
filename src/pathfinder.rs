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

    fn get_path(&self) -> Vec<Vector2<usize>>;

    fn get_visited(&self) -> Vec<Vec<bool>>;

    fn get_state(&self) -> Vec<Vec<Option<String>>>;

    fn deinit(&mut self);
}
