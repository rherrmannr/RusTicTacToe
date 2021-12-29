use crate::tic_toc::game_field::GameField;

pub trait UI {
    fn display(&mut self, game_field: &GameField);
    fn process_input(&self) -> Option<(usize, usize)>;
}
