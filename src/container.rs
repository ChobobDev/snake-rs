use cursive::{Printer, View};

pub struct Container {}

impl Container {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Container {
    fn draw(&self, printer: &Printer) {
                  }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(44, 22)
    }
}
