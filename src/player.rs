use ::bracket_lib::prelude::*;
use crate::window;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 5,
            y: window::SCREEN_HEIGHT / 2,
            velocity: 2.0,
        }
    }
    pub fn input(&mut self, key: Option<VirtualKeyCode>) {
        if let Some(key) = key {
            match key {
                VirtualKeyCode::Space => self.flap(),
                _ => {}
            }
        }
    }

    pub fn flap(&mut self) {
        self.velocity = -2.0;
    }

    pub fn gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn frame_tick(&mut self, ctx: &mut BTerm) {
        // Environment Systems
        self.gravity();
    }

    pub fn tick(&mut self, ctx: &mut BTerm) {
        // Input Systems
        self.input(ctx.key);
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        // The x position of the player counts how far they've gone, but visually
        // it should always be presened as being in the same column
        ctx.set(
            self.x,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@'),
        );
    }
}