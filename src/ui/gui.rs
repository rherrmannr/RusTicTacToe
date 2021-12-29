use super::ui::UI;
use crate::tic_toc::game_field::GameField;

impl GUI {
    pub fn new() -> GUI {
        GUI {}
    }
}

impl UI for GUI {
    fn display(&mut self, game_field: &GameField) {}
    fn process_input(&self) -> Option<(usize, usize)> {}
}
