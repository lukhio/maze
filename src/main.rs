extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 600;
const SQUARE_SIZE: u32 = 5;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for i in (0..SCREEN_HEIGHT).step_by(SQUARE_SIZE as usize) {
            for j in (0..SCREEN_WIDTH).step_by(SQUARE_SIZE as usize) {
                if i == j {
                    let rect = Rect::new(i.try_into().unwrap(),
                                         j.try_into().unwrap(),
                                         SQUARE_SIZE,
                                         SQUARE_SIZE);
                    canvas.fill_rect(rect).unwrap();
                    canvas.draw_rect(rect).unwrap();
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
