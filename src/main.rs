extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::{ Rect, Point };
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::collections::HashSet;
use rand::thread_rng;
use rand::seq::SliceRandom;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 800;
const SQUARE_SIZE: usize = 2;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Wall {
    Up,
    Down
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Node {
    visited: bool,
    north: Wall,
    east: Wall,
    south: Wall,
    west: Wall,
}

impl Node {
    fn new() -> Self {
        Node {
            visited: false,
            north: Wall::Up,
            east: Wall::Up,
            south: Wall::Up,
            west: Wall::Up,
        }
    }
}

struct Maze {
    maze: [Node; (SCREEN_HEIGHT * SCREEN_WIDTH) / SQUARE_SIZE],
}

impl Maze {
    fn get(&mut self, x: usize, y: usize) -> Node {
        let cst = SCREEN_WIDTH / SQUARE_SIZE;
        // this will panic if we are outside of bounds
        self.maze[x + (y * cst)]
    }

    fn draw<T: sdl2::render::RenderTarget>(&mut self,
                                           canvas: &mut Canvas<T>,
                                           highlight: Option<(usize, usize)>) {
        canvas.clear();

        for x in 0..(SCREEN_WIDTH / SQUARE_SIZE) {
            for y in 0..(SCREEN_HEIGHT / SQUARE_SIZE) {
                canvas.set_draw_color(Color::RGB(255, 255, 255));

                let rect = Rect::new((x * SQUARE_SIZE).try_into().unwrap(),
                                     (y * SQUARE_SIZE).try_into().unwrap(),
                                     SQUARE_SIZE.try_into().unwrap(),
                                     SQUARE_SIZE.try_into().unwrap());

                canvas.fill_rect(rect).unwrap();
                canvas.draw_rect(rect).unwrap();

                // Draw highlight in reg
                if let Some((i, j)) = highlight {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    let rect = Rect::new((i * SQUARE_SIZE).try_into().unwrap(),
                                         (j * SQUARE_SIZE).try_into().unwrap(),
                                         SQUARE_SIZE.try_into().unwrap(),
                                         SQUARE_SIZE.try_into().unwrap());
                    canvas.fill_rect(rect).unwrap();
                    canvas.draw_rect(rect).unwrap();
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
            }
        }

        for x in 0..(SCREEN_WIDTH / SQUARE_SIZE) {
            for y in 0..(SCREEN_HEIGHT / SQUARE_SIZE) {
                let rect = Rect::new((x * SQUARE_SIZE).try_into().unwrap(),
                                     (y * SQUARE_SIZE).try_into().unwrap(),
                                     SQUARE_SIZE.try_into().unwrap(),
                                     SQUARE_SIZE.try_into().unwrap());

                // Draw walls in black
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                if self.get(x, y).north == Wall::Up {
                    let border_start = Point::new(rect.left(), rect.top());
                    let border_end = Point::new(rect.right(), rect.top());
                    canvas.draw_line(border_start, border_end).unwrap();
                }
                if self.get(x, y).east == Wall::Up {
                    let border_start = Point::new(rect.right(), rect.top());
                    let border_end = Point::new(rect.right(), rect.bottom());
                    canvas.draw_line(border_start, border_end).unwrap();
                }
                if self.get(x, y).south == Wall::Up {
                    let border_start = Point::new(rect.left(), rect.bottom());
                    let border_end = Point::new(rect.right(), rect.bottom());
                    canvas.draw_line(border_start, border_end).unwrap();
                }
                if self.get(x, y).west == Wall::Up {
                    let border_start = Point::new(rect.left(), rect.top());
                    let border_end = Point::new(rect.left(), rect.bottom());
                    canvas.draw_line(border_start, border_end).unwrap();
                }

            }
        }
        canvas.present();
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if x > 0
                && x < SCREEN_WIDTH / SQUARE_SIZE
                && y > 0
                && y < SCREEN_HEIGHT / SQUARE_SIZE {
            neighbors.push((x - 1, y));
            neighbors.push((x, y - 1));
            neighbors.push((x + 1, y));
            neighbors.push((x, y + 1));
        }

        if x == 0 {
            neighbors.push((x + 1, y));
            if y > 0 {
                neighbors.push((x, y - 1));
            }
            if y < SCREEN_HEIGHT / SQUARE_SIZE {
                neighbors.push((x, y + 1));
            }
        }

        if y == 0 {
            neighbors.push((x, y + 1));
            if x > 0 {
                neighbors.push((x - 1, y));
            }
            if x < SCREEN_WIDTH / SQUARE_SIZE {
                neighbors.push((x + 1, y));
            }
        }

        neighbors.sort();
        neighbors.dedup();
        neighbors
    }
}

pub fn main() {
    // Init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("maze",
                                        (SCREEN_WIDTH).try_into().unwrap(),
                                        (SCREEN_HEIGHT).try_into().unwrap())
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Draw background in black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = thread_rng();

    // Draw maze
    let mut maze = Maze {
        maze: [Node::new(); (SCREEN_HEIGHT * SCREEN_WIDTH) / SQUARE_SIZE]
    };
    maze.draw(&mut canvas, Some((0, 0)));

    let mut root = maze.get(0, 0);
    root.visited = true;
    let mut stack = vec![(0, 0)];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while ! stack.is_empty() {
        // Get a node
        let current_coords = stack.pop().unwrap();
        visited.insert(current_coords);

        // Get unvisited neighbors
        let mut neighbors: Vec<(usize, usize)> = maze.get_neighbors(current_coords.0,
                                                                    current_coords.1)
            .into_iter()
            .filter(|coords| ! visited.contains(&(coords.0, coords.1)))
            .collect();

        // If we have some unvisited visitors, visit a random one
        if ! neighbors.is_empty() {
            let chosen_one = neighbors.choose_mut(&mut rng).unwrap();

            // maze.update_walls(current_coords, *chosen_one);
            let cst = SCREEN_WIDTH / SQUARE_SIZE;
            if current_coords.0 == chosen_one.0 {
                if current_coords.1 > chosen_one.1 {
                    maze.maze[current_coords.0 + (current_coords.1 * cst)].north = Wall::Down;
                    maze.maze[chosen_one.0 + (chosen_one.1 * cst)].south = Wall::Down;
                } else {
                    maze.maze[current_coords.0 + (current_coords.1 * cst)].south = Wall::Down;
                    maze.maze[chosen_one.0 + (chosen_one.1 * cst)].north = Wall::Down;
                }
            } else if current_coords.0 > chosen_one.0 {
                maze.maze[current_coords.0 + (current_coords.1 * cst)].west = Wall::Down;
                maze.maze[chosen_one.0 + (chosen_one.1 * cst)].east = Wall::Down;
            } else {
                maze.maze[current_coords.0 + (current_coords.1 * cst)].east = Wall::Down;
                maze.maze[chosen_one.0 + (chosen_one.1 * cst)].west = Wall::Down;
            }

            stack.push(current_coords);
            stack.push(*chosen_one);
            visited.insert(*chosen_one);
        }

        // maze.draw(&mut canvas, Some(current_coords));
        // ::std::thread::sleep(Duration::from_millis(10));
    }
    maze.draw(&mut canvas, None);

    'running: loop {

        // Wait for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
