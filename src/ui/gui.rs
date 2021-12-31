use super::ui_base::*;

use crate::tic_tac_toe::game_field::{GameField, State};
use crate::tic_tac_toe::player::Player;

extern crate sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;
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
    pub const FONT: Color = Color::RGB(195, 195, 195);
}

pub struct Gui {
    canvas: Canvas<Window>,
    events: EventPump,
    ttf_context: Sdl2TtfContext,
}

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl Gui {
    pub fn new() -> Gui {
        let sdl_context = sdl2::init().unwrap();

        let video_subsys = sdl_context.video().unwrap();

        let mut window = video_subsys
            .window("RusTicTacToe", screen::WIDTH, screen::HEIGHT)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let window_icon = Surface::from_file("logos/tic-tac-toe_39453.png").unwrap();
        window.set_icon(window_icon);

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        canvas.set_draw_color(color::BACKGROUND);
        canvas.clear();
        canvas.present();


        let events = sdl_context.event_pump().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        Gui {
            canvas,
            events,
            ttf_context,
        }
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
                    crate::tic_tac_toe::player::Sign::X => self.draw_x(game_field, (i, j)),
                    crate::tic_tac_toe::player::Sign::O => self.draw_o(game_field, (i, j)),
                    _ => {}
                }
            }
        }
    }

    fn get_centered_rect(
        &self,
        rect_width: u32,
        rect_height: u32,
        cons_width: u32,
        cons_height: u32,
    ) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = (self.screen_width() as i32 - w) / 2;
        let cy = (self.screen_height() as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }

    fn draw_draw(&mut self) {
        self.draw_text("It's a draw!".to_string());
    }

    fn draw_player_has_won(&mut self, player: &Player) {
        self.draw_text(format!("Player {} has won!", player.sign()));
    }

    fn draw_text(&mut self, text: String) {
        let mut padding = 64;
        // dynamic padding
        if self.screen_width() < padding || self.screen_height() < padding {
            padding = std::cmp::min(self.screen_width(), self.screen_height());
        }
        let texture_creator = self.canvas.texture_creator();
        let mut font = self
            .ttf_context
            .load_font("fonts/pilotcommand.ttf", 128)
            .unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
        let surface = font
            .render(&text)
            .blended(color::FONT)
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = self.get_centered_rect(
            width,
            height,
            self.screen_width() - padding,
            self.screen_height() - padding,
        );
        self.canvas.copy(&texture, None, Some(target)).unwrap();
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
        let offset = (radius as f64 / std::f64::consts::SQRT_2) as i16;
        self.canvas
            .line(x - offset, y - offset, x + offset, y + offset, color::X)
            .unwrap();
        self.canvas
            .line(x + offset, y - offset, x - offset, y + offset, color::X)
            .unwrap();
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
        match game_field.get_state() {
            State::Winner(winner) => self.draw_player_has_won(&winner),
            State::Draw => self.draw_draw(),
            State::Playing => {}
        }
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
                    if keycode == Keycode::R {
                        return Event::Restart;
                    }
                }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => match game_field.get_state() {
                    State::Playing => {
                        return Event::Point(self.coordinates_as_point(x, y, game_field))
                    }
                    State::Draw | State::Winner(_) => return Event::Restart,
                },

                _ => {}
            }
        }
        Event::None
    }
}
