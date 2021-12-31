use crate::tic_toc::game_field::GameField;

pub enum Mode {
    Gui,
    Cli,
}

pub enum Event {
    Quit,
    Point((usize, usize)),
    Restart,
    None,
}

pub trait UI {
    fn display(&mut self, game_field: &GameField);
    fn process_input(&mut self, game_field: &GameField) -> Event;
}
