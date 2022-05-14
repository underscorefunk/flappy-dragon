use crate::window;
use ::bracket_lib::prelude::*;

pub struct Obstacle {
    pub x: i32,
    pub gap_start: i32,
    pub gap_end: i32,
}

impl Obstacle {

    pub fn new(start: i32, width: i32) -> Self {
        Self {
            x: window::SCREEN_WIDTH,
            gap_start: start,
            gap_end: start + width,
        }
    }

    pub fn frame_tick(&mut self) {
        self.x -= 1;
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        for y in 0..window::SCREEN_HEIGHT {
            if y < self.gap_start || self.gap_end < y {
                ctx.set(
                    self.x,
                    y,
                    RED,
                    ORANGE,
                    to_cp437('X'),
                );
            }
        }
    }
}