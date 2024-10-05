use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::pathfinder::Pathfinder;
use crate::vector::Vector2;

const DIAGONAL_COST: i32 = 14;
const STRAIGHT_COST: i32 = 10;

#[derive(Clone, Eq)]
struct AStarNode {
    pos: Vector2<usize>,
    g_cost: i32, // distance from starting node
    h_cost: i32, // distance from ending node
}

impl AStarNode {
    fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost() == other.f_cost()
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost())
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct AStarNodeData {
    g_cost: i32,
    prev_pos: Option<Vector2<usize>>,
}

fn distance(pos1: &Vector2<usize>, pos2: &Vector2<usize>) -> i32 {
    let dx = (pos1.x as i32 - pos2.x as i32).abs();
    let dy = (pos1.y as i32 - pos2.y as i32).abs();

    let min = dx.min(dy);
    let max = dx.max(dy);

    min * DIAGONAL_COST + max * STRAIGHT_COST
}

pub struct AStar {
    frontier: BinaryHeap<AStarNode>,
    curr_node: Option<AStarNode>,
    state: Vec<Vec<AStarNodeData>>,

    rows: i32,
    cols: i32,
    end_pos: Vector2<usize>,
    walls: Vec<Vec<bool>>,
}

impl Default for AStar {
    fn default() -> Self {
        Self {
            frontier: BinaryHeap::new(),
            curr_node: None,
            state: vec![],
            rows: 0,
            cols: 0,
            end_pos: Vector2 { x: 0, y: 0 },
            walls: vec![],
        }
    }
}

impl Pathfinder for AStar {
    fn init(
        &mut self,
        rows: i32,
        cols: i32,
        start_pos: Vector2<usize>,
        end_pos: Vector2<usize>,
        walls: Vec<Vec<bool>>,
    ) {
        self.frontier.clear();
        self.frontier.push(AStarNode {
            pos: start_pos,
            g_cost: 0,
            h_cost: distance(&start_pos, &end_pos),
        });

        self.state = vec![
            vec![
                AStarNodeData {
                    g_cost: i32::MAX,
                    prev_pos: None,
                };
                cols as usize
            ];
            rows as usize
        ];
        self.state[start_pos.y][start_pos.x] = AStarNodeData {
            g_cost: 0,
            prev_pos: None,
        };

        self.rows = rows;
        self.cols = cols;
        self.end_pos = end_pos;
        self.walls = walls;
    }

    fn step(&mut self) -> Option<Vector2<usize>> {
        let curr_node = match self.frontier.pop() {
            Some(node) => node,
            None => {
                // no path found
                self.curr_node = None;
                return None;
            }
        };

        self.curr_node = Some(curr_node.clone());

        // found end node
        if curr_node.pos == self.end_pos {
            return Some(self.end_pos);
        }

        // cannot move these values out of self
        let state = self.state.clone();
        let rows = self.rows;
        let cols = self.cols;
        let end_pos = self.end_pos;
        let walls = &self.walls;

        (-1..=1)
            .flat_map(move |i: i32| {
                let moved_state = state.clone();

                (-1..=1).flat_map(move |j: i32| {
                    if i == 0 && j == 0 {
                        return None;
                    }

                    let x = curr_node.pos.x as i32 + i;
                    let y = curr_node.pos.y as i32 + j;

                    if x < 0 || y < 0 || x >= cols || y >= rows {
                        return None;
                    }

                    if walls[y as usize][x as usize] {
                        return None;
                    }

                    let pos = Vector2 {
                        x: x as usize,
                        y: y as usize,
                    };

                    let g_cost = curr_node.g_cost
                        + if i.abs() + j.abs() == 2 {
                            DIAGONAL_COST
                        } else {
                            STRAIGHT_COST
                        };

                    // don't backtrack
                    if g_cost >= moved_state[pos.y][pos.x].g_cost {
                        return None;
                    }

                    Some(AStarNode {
                        pos,
                        g_cost,
                        h_cost: distance(&end_pos, &pos),
                    })
                })
            })
            .for_each(|node| {
                self.state[node.pos.y][node.pos.x] = AStarNodeData {
                    g_cost: node.g_cost,
                    prev_pos: Some(curr_node.pos),
                };
                self.frontier.push(node)
            });

        Some(curr_node.pos)
    }

    fn get_frontier(&self) -> Vec<Vector2<usize>> {
        self.frontier
            .clone()
            .into_iter()
            .map(|node| node.pos)
            .collect()
    }

    fn get_path(&self) -> Vec<Vector2<usize>> {
        let mut curr_pos = match &self.curr_node {
            Some(node) => node.pos,
            None => return vec![],
        };

        let mut path = vec![curr_pos];

        while let Some(pos) = self.state[curr_pos.y][curr_pos.x].prev_pos {
            curr_pos = pos;
            path.push(curr_pos);
        }

        path
    }

    fn get_visited(&self) -> Vec<Vec<bool>> {
        self.state
            .iter()
            .map(|row| row.iter().map(|v| v.g_cost != i32::MAX).collect())
            .collect()
    }

    fn deinit(&mut self) {
        self.frontier.clear();
        self.curr_node = None;
        self.state.clear();

        self.rows = 0;
        self.cols = 0;
        self.end_pos = Vector2 { x: 0, y: 0 };
        self.walls = vec![];
    }
}
