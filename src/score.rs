use bracket_lib::prelude::*;
use crate::window;

// You can make a struct that just contains one value
// instead of having properties. Think of it as a type alias.
// You can access it's value using dot notation and the index,
// as if it's a tuple. i.e. `my_score.0`
pub struct Score(u32);

impl Score {

    pub fn new() -> Self {
        Self(0)
    }

    pub fn count(&mut self) {
        self.0 += 1;
    }

    pub fn frame_tick(&mut self) {
        self.count();
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.print_right(
            window::SCREEN_WIDTH - 2,
            2,
            format!("Score: {}", self.0),
        );
    }
}