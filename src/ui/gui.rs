use super::ui::UI;

use crate::tic_toc::game_field::GameField;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels, EventPump};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct GUI {
    canvas: Canvas<Window>,
    events: EventPump,
}

impl GUI {
    pub fn new() -> GUI {
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

        let gui = GUI { canvas, events };
        gui
    }
}

impl UI for GUI {
    fn display(&mut self, _game_field: &GameField) {}
    fn process_input(&mut self) -> Option<(usize, usize)> {
        for event in self.events.poll_iter() {
            match event {
                // Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        println!("We should escape the game here..");
                    } else {
                        println!("Some key has been pressed..");
                    }
                    self.canvas.present();
                }

                Event::MouseButtonDown { x, y, .. } => {
                    println!("mouse btn down at ({},{})", x, y);
                    self.canvas.present();
                }

                _ => {}
            }
        }
        None
    }
}
