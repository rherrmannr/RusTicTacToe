use crate::tic_toc::game_field::GameField;

pub enum Mode {
    GUI,
    CLI,
}
pub trait UI {
    fn display(&mut self, game_field: &GameField);
    fn process_input(&mut self) -> Option<(usize, usize)>;
}
