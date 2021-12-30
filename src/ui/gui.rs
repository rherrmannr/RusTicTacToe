use super::ui_base::*;

use crate::tic_toc::game_field::GameField;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels, EventPump};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct Gui {
    canvas: Canvas<Window>,
    events: EventPump,
}

impl Gui {
    pub fn new() -> Gui {
        let sdl_context = sdl2::init().unwrap();

        let video_subsys = sdl_context.video().unwrap();

        let window = video_subsys
            .window("Rusty Tic Tac Toe", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let events = sdl_context.event_pump().unwrap();

        Gui { canvas, events }
    }
}

impl UI for Gui {
    fn display(&mut self, _game_field: &GameField) {
        self.canvas.present();
    }

    fn process_input(&mut self) -> Event {
        for event in self.events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return Event::Quit,
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        println!("We should escape the game here..");
                    } else {
                        println!("Some key has been pressed..");
                    }
                }

                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    println!("mouse btn down at ({},{})", x, y);
                }

                _ => {}
            }
        }
        Event::None
    }
}
