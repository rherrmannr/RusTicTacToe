use super::ui_base::*;

use crate::tic_toc::game_field::GameField;

extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

mod screen {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;
}

mod color {
    use sdl2::pixels::Color;

    pub const BACKGROUND: Color = Color::RGB(30, 30, 30);
    pub const FIELD: Color = Color::RGB(205, 205, 205);
    pub const O: Color = Color::RGB(255, 95, 31);
    pub const X: Color = Color::RGB(255, 16, 240);
}

pub struct Gui {
    canvas: Canvas<Window>,
    events: EventPump,
}

impl Gui {
    pub fn new() -> Gui {
        let sdl_context = sdl2::init().unwrap();

        let video_subsys = sdl_context.video().unwrap();

        let window = video_subsys
            .window("RusTicTacToe", screen::WIDTH, screen::HEIGHT)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(color::BACKGROUND);
        canvas.clear();
        canvas.present();
        let events = sdl_context.event_pump().unwrap();

        Gui { canvas, events }
    }

    fn screen_width(&self) -> u32 {
        self.canvas.window().size().0
    }

    fn screen_height(&self) -> u32 {
        self.canvas.window().size().1
    }

    fn coordinates_as_point(&mut self, x: i32, y: i32, game_field: &GameField) -> (usize, usize) {
        let rect_width = self.screen_width() / game_field.size() as u32;
        let rect_heigth = self.screen_height() / game_field.size() as u32;
        let column = x / rect_width as i32;
        let row = y / rect_heigth as i32;
        (row as usize, column as usize)
    }

    fn draw_field(&mut self, game_field: &GameField) {
        for section in 1..game_field.size() {
            let y = ((self.screen_height() / game_field.size() as u32) * section as u32) as i16;
            let x = ((self.screen_width() / game_field.size() as u32) * section as u32) as i16;

            self.canvas
                .line(0, y, self.screen_width() as i16, y as i16, color::FIELD)
                .unwrap();
            self.canvas
                .line(x, 0, x, self.screen_height() as i16 as i16, color::FIELD)
                .unwrap();
        }
    }

    fn draw_signs(&mut self, game_field: &GameField) {
        for (i, signs) in game_field.get_field().iter().enumerate() {
            for (j, &sign) in signs.iter().enumerate() {
                match sign {
                    crate::tic_toc::player::Sign::X => self.draw_x(game_field, (i, j)),
                    crate::tic_toc::player::Sign::O => self.draw_o(game_field, (i, j)),
                    _ => {}
                }
            }
        }
    }
    fn draw_game_field(&mut self, game_field: &GameField) {
        self.draw_field(game_field);
        self.draw_signs(game_field);
    }

    fn draw_x(&mut self, game_field: &GameField, point: (usize, usize)) {
        let rect_width = self.screen_width() / game_field.size() as u32;
        let rect_heigth = self.screen_height() / game_field.size() as u32;

        let y = (rect_heigth * point.0 as u32) as i16 + (rect_heigth / 2) as i16;
        let x = (rect_width * point.1 as u32) as i16 + (rect_width / 2) as i16;
        let radius = (std::cmp::min(rect_heigth, rect_width) * 2 / 10) as i16;

        self.canvas.aa_circle(x, y, radius, color::X).unwrap();
    }

    fn draw_o(&mut self, game_field: &GameField, point: (usize, usize)) {
        let rect_width = self.screen_width() / game_field.size() as u32;
        let rect_heigth = self.screen_height() / game_field.size() as u32;

        let y = (rect_heigth * point.0 as u32) as i16 + (rect_heigth / 2) as i16;
        let x = (rect_width * point.1 as u32) as i16 + (rect_width / 2) as i16;
        let radius = (std::cmp::min(rect_heigth, rect_width) * 2 / 10) as i16;

        self.canvas.aa_circle(x, y, radius, color::O).unwrap();
    }
}

impl UI for Gui {
    fn display(&mut self, game_field: &GameField) {
        self.canvas.set_draw_color(color::BACKGROUND);
        self.canvas.clear();
        self.draw_game_field(game_field);
        self.canvas.present();
    }

    fn process_input(&mut self, game_field: &GameField) -> Event {
        for event in self.events.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return Event::Quit,
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        return Event::Quit;
                    }
                }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    return Event::Point(self.coordinates_as_point(x, y, game_field));
                }

                _ => {}
            }
        }
        Event::None
    }
}
