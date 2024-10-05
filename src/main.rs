pub mod algorithms;
pub mod pathfinder;
pub mod vector;

use algorithms::a_star::AStar;
use macroquad::prelude::*;
use pathfinder::Pathfinder;
use vector::Vector2;

const DEBUG: bool = false;

#[macroquad::main("Pathfinding")]
async fn main() {
    let a_star = Box::<AStar>::default();

    let mut rows = 20i32;
    let mut cols = 20i32;
    let mut start_pos = Vector2 { x: 3, y: 3 };
    let mut end_pos = Vector2 { x: 16, y: 16 };
    let mut walls = vec![vec![false; cols as usize]; rows as usize];

    let mut curr_pos = start_pos;

    let mut pathfinders: Vec<Box<dyn Pathfinder>> = vec![a_star];
    let pathfinder_idx = 0;

    let mut fps = 1f64;
    let mut last_time = get_time();
    let mut running = false;
    let mut instant = false;

    let mut needs_reset = true;
    let mut no_path = false;

    loop {
        // resizing
        if is_key_pressed(KeyCode::Up) && rows > 0 {
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
        if is_key_pressed(KeyCode::Down) {
            rows += 1;
            walls.push(vec![false; cols as usize]);
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
        if is_key_pressed(KeyCode::Right) {
            cols += 1;
            walls.iter_mut().for_each(|row| {
                row.push(false);
            });
            needs_reset = true;
        }

        // fps control
        if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Equal) {
            fps *= 2.0;
        }
        if is_key_pressed(KeyCode::Minus) {
            fps /= 2.0;
        }

        // pathfinding control
        if is_key_pressed(KeyCode::Space) {
            if curr_pos == end_pos || no_path {
                needs_reset = true;
            } else {
                running = !running;
            }
        }
        if is_key_pressed(KeyCode::Enter) {
            instant = !instant;
        }

        let cell_size = (screen_width() / cols as f32).min(screen_height() / rows as f32);

        // placing cells
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
            // TODO: check if wall is on important cell
            if x < cols as usize && y < rows as usize {
                walls[y][x] = true;
                needs_reset = true;
            }
        } else if is_mouse_button_down(MouseButton::Right) {
            let x = (mouse_position().0 / cell_size).floor() as usize;
            let y = (mouse_position().1 / cell_size).floor() as usize;
            if x < cols as usize && y < rows as usize {
                walls[y][x] = false;
                needs_reset = true;
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            walls = vec![vec![false; cols as usize]; rows as usize];
        }

        if needs_reset {
            needs_reset = false;
            curr_pos = start_pos;
            running = false;
            no_path = false;

            pathfinders[pathfinder_idx].init(rows, cols, start_pos, end_pos, walls.clone());
        }

        if get_time() > last_time + 1.0 / fps && running {
            last_time = get_time();

            loop {
                match pathfinders[pathfinder_idx].step() {
                    Some(pos) => {
                        curr_pos = pos;
                        if curr_pos == end_pos {
                            running = false;
                        }
                    }
                    None => {
                        curr_pos = start_pos;
                        running = false;
                        no_path = true;
                    }
                }

                if !instant || !running {
                    break;
                }
            }
        }

        clear_background(BLACK);

        pathfinders[pathfinder_idx]
            .get_visited()
            .into_iter()
            .enumerate()
            .for_each(|(i, row)| {
                row.into_iter().enumerate().for_each(|(j, v)| {
                    if v {
                        draw_cell_coords(
                            j,
                            i,
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
                draw_cell(
                    pos,
                    cell_size,
                    Color {
                        r: 0.6,
                        g: 0.6,
                        b: 0.6,
                        a: 1.0,
                    },
                );
            });

        pathfinders[pathfinder_idx]
            .get_path()
            .into_iter()
            .for_each(|pos| {
                draw_cell(pos, cell_size, PURPLE);
            });

        draw_cell(curr_pos, cell_size, WHITE);
        draw_cell(start_pos, cell_size, BLUE);
        draw_cell(end_pos, cell_size, RED);

        if DEBUG {
            pathfinders[pathfinder_idx]
                .get_state()
                .iter()
                .enumerate()
                .for_each(|(i, row)| {
                    row.iter().enumerate().for_each(|(j, v)| {
                        if let Some(v) = v {
                            draw_text(
                                &v.to_string(),
                                j as f32 * cell_size + cell_size * 0.1,
                                i as f32 * cell_size + cell_size * 0.6,
                                12.0,
                                WHITE,
                            );
                        }
                    })
                });
        }

        walls.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, v)| {
                if *v {
                    draw_cell_coords(j, i, cell_size, WHITE);
                }
            })
        });

        draw_borders(rows, cols, cell_size);

        draw_text(
            &format!(
                "FPS: {}",
                if instant {
                    "Instant".to_string()
                } else {
                    fps.to_string()
                }
            ),
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn draw_cell(pos: Vector2<usize>, cell_size: f32, colour: Color) {
    draw_rectangle(
        pos.x as f32 * cell_size,
        pos.y as f32 * cell_size,
        cell_size,
        cell_size,
        colour,
    );
}

fn draw_cell_coords(x: usize, y: usize, cell_size: f32, colour: Color) {
    draw_rectangle(
        x as f32 * cell_size,
        y as f32 * cell_size,
        cell_size,
        cell_size,
        colour,
    );
}

fn draw_borders(rows: i32, cols: i32, cell_size: f32) {
    (0..=rows).for_each(|i| {
        let y = i as f32 * cell_size;

        draw_line(0.0, y, cols as f32 * cell_size, y, 2.0, GRAY);
    });

    (0..=cols).for_each(|j| {
        let x = j as f32 * cell_size;

        draw_line(x, 0.0, x, rows as f32 * cell_size, 2.0, GRAY);
    });
}
