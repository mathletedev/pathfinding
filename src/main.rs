pub mod algorithms;
pub mod pathfinder;
pub mod vector;

use algorithms::a_star::AStar;
use macroquad::prelude::*;
use pathfinder::Pathfinder;
use vector::Vector2;

#[macroquad::main("Pathfinding")]
async fn main() {
    let a_star = Box::<AStar>::default();

    let mut rows = 20i32;
    let mut cols = 20i32;
    let mut start_pos = Vector2 { x: 0, y: 0 };
    let mut end_pos = Vector2 { x: 19, y: 19 };
    let mut walls = vec![vec![false; cols as usize]; rows as usize];

    let mut curr_pos = start_pos;

    let mut pathfinders: Vec<Box<dyn Pathfinder>> = vec![a_star];
    let pathfinder_idx = 0;

    let mut fps = 100f64;
    let mut last_time = get_time();
    let mut running = false;

    let mut needs_reset = true;

    loop {
        if is_key_pressed(KeyCode::Up) {
            rows += 1;
            needs_reset = true;
        }
        if is_key_pressed(KeyCode::Down) && rows > 0 {
            rows -= 1;
            walls.pop();
            if start_pos.y >= rows as usize {
                start_pos.y = rows as usize - 1;
            }
            if end_pos.y >= rows as usize {
                end_pos.y = rows as usize - 1;
            }
            needs_reset = true;
        }
        if is_key_pressed(KeyCode::Right) {
            cols += 1;
            needs_reset = true;
        }
        if is_key_pressed(KeyCode::Left) && cols > 0 {
            cols -= 1;
            walls.iter_mut().for_each(|row| {
                row.pop();
            });
            if start_pos.x >= cols as usize {
                start_pos.x = cols as usize - 1;
            }
            if end_pos.x >= cols as usize {
                end_pos.x = cols as usize - 1;
            }
            needs_reset = true;
        }

        if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Equal) {
            fps += 1.0;
        }
        if is_key_pressed(KeyCode::Minus) {
            fps -= 1.0;
        }

        if is_key_pressed(KeyCode::Space) {
            if curr_pos == end_pos {
                needs_reset = true;
            } else {
                running = !running;
            }
        }

        let cell_size = (screen_width() / cols as f32).min(screen_height() / rows as f32);

        if is_key_pressed(KeyCode::S) {
            start_pos = Vector2 {
                x: (mouse_position().0 / cell_size).floor() as usize,
                y: (mouse_position().1 / cell_size).floor() as usize,
            };
            curr_pos = start_pos;
            needs_reset = true;
        }
        if is_key_pressed(KeyCode::E) {
            end_pos = Vector2 {
                x: (mouse_position().0 / cell_size).floor() as usize,
                y: (mouse_position().1 / cell_size).floor() as usize,
            };
            needs_reset = true;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let x = (mouse_position().0 / cell_size).floor() as usize;
            let y = (mouse_position().1 / cell_size).floor() as usize;
            if x != start_pos.x || y != start_pos.y && x < cols as usize && y < rows as usize {
                walls[x][y] = true;
                needs_reset = true;
            }
        } else if is_mouse_button_down(MouseButton::Right) {
            let x = (mouse_position().0 / cell_size).floor() as usize;
            let y = (mouse_position().1 / cell_size).floor() as usize;
            if x != start_pos.x || y != start_pos.y && x < cols as usize && y < rows as usize {
                walls[x][y] = false;
                needs_reset = true;
            }
        }

        if needs_reset {
            needs_reset = false;
            curr_pos = start_pos;
            running = false;

            pathfinders[pathfinder_idx].init(rows, cols, start_pos, end_pos, walls.clone());
        }

        if get_time() > last_time + 1.0 / fps && running {
            last_time = get_time();

            match pathfinders[pathfinder_idx].step() {
                Some(pos) => curr_pos = pos,
                None => {
                    curr_pos = end_pos;
                    running = false;
                }
            }
        }

        clear_background(BLACK);

        walls.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, v)| {
                if *v {
                    draw_rectangle(
                        i as f32 * cell_size,
                        j as f32 * cell_size,
                        cell_size,
                        cell_size,
                        WHITE,
                    );
                }
            })
        });

        pathfinders[pathfinder_idx]
            .get_visited()
            .into_iter()
            .enumerate()
            .for_each(|(i, row)| {
                row.into_iter().enumerate().for_each(|(j, v)| {
                    if v {
                        draw_rectangle(
                            i as f32 * cell_size,
                            j as f32 * cell_size,
                            cell_size,
                            cell_size,
                            Color {
                                r: 0.3,
                                g: 0.3,
                                b: 0.3,
                                a: 1.0,
                            },
                        );
                    }
                })
            });

        pathfinders[pathfinder_idx]
            .get_frontier()
            .into_iter()
            .for_each(|pos| {
                let x = pos.x as f32 * cell_size;
                let y = pos.y as f32 * cell_size;

                draw_rectangle(
                    x,
                    y,
                    cell_size,
                    cell_size,
                    Color {
                        r: 0.6,
                        g: 0.6,
                        b: 0.6,
                        a: 1.0,
                    },
                );
            });

        draw_rectangle(
            start_pos.x as f32 * cell_size,
            start_pos.y as f32 * cell_size,
            cell_size,
            cell_size,
            BLUE,
        );
        draw_rectangle(
            end_pos.x as f32 * cell_size,
            end_pos.y as f32 * cell_size,
            cell_size,
            cell_size,
            RED,
        );

        draw_rectangle(
            curr_pos.x as f32 * cell_size,
            curr_pos.y as f32 * cell_size,
            cell_size,
            cell_size,
            WHITE,
        );

        (0..=rows).for_each(|i| {
            let y = i as f32 * cell_size;

            draw_line(0.0, y, cols as f32 * cell_size, y, 2.0, GRAY);
        });

        (0..=cols).for_each(|j| {
            let x = j as f32 * cell_size;

            draw_line(x, 0.0, x, rows as f32 * cell_size, 2.0, GRAY);
        });

        next_frame().await;
    }
}
