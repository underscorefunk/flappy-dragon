use ::bracket_lib::prelude::*;
use crate::obstacle::Obstacle;

// import all (denoted by the *) from the payer and score modules
// into the current module (main module) scope for use.
//
// use player::Player allows use to write 'Player' as the type for the 'player'
// in the State struct.
//
// We use 'crate' to specify that the module 'player' exists in our current crate
// This is satisfied by having `mod player` in main.rs
use crate::player::*;
use crate::score::*;
use crate::obstacle::*;
use crate::window;

const FRAME_DURATION: f32 = 50.0;

enum Mode {
    Playing,
    Dead
}

pub struct State {
    mode: Mode,
    player: Player,
    score: Score,
    frame_count: i32,
    frame_time: f32,
    obstacles: Vec<Obstacle>,

}

impl State {
    pub fn new() -> Self {
        State {
            mode: Mode::Playing,
            score: Score::new(),
            player: Player::new(),
            frame_count: 0,
            frame_time: 0.0,
            obstacles: vec![
                Obstacle::new(6,10)
            ]
        }
    }
    pub fn maybe_add_obstacle(&mut self) {
        if self.frame_count % 50 == 0 {
            let mut random = RandomNumberGenerator::new();
            let width = 10;
            let start = random.range(0, window::SCREEN_HEIGHT - width);
            self.obstacles.push(
                Obstacle::new(start,width)
            )
        }
    }

    pub fn maybe_dead(&mut self) {

        if self.player.y > window::SCREEN_HEIGHT {
            self.mode = Mode::Dead;
            return;
        }

        for o in &self.obstacles {
            if self.player.x == o.x // If the player is in the same column as the obstacle
                && (self.player.y < o.gap_start || o.gap_end < self.player.y) {
                self.mode = Mode::Dead;
            }
        }
    }

    pub fn playing_tick(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        // Grab how long it's been since the last tick
        // and add it to the running counter
        self.frame_time += ctx.frame_time_ms;

        // Perform ticks for components at the cpu rate (as quickly as possible)
        // Usually for IO
        self.player.tick(ctx);

        // Perform component ticks that should only happen at the frame
        // clock rate
        if self.frame_time > FRAME_DURATION {
            // Reset the counter because we're doing a quantized "frame tick"
            self.frame_time = 0.0;
            self.frame_count += 1;

            self.maybe_add_obstacle();

            self.score.frame_tick();
            self.player.frame_tick(ctx);
            for o in &mut self.obstacles {
                o.frame_tick()
            }
        }

        // Render components
        self.score.render(ctx);
        self.player.render(ctx);
        for o in &mut self.obstacles {
            o.render(ctx)
        }
        self.maybe_dead();
    }

    pub fn dead_tick(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(RED);
        ctx.print_centered(window::SCREEN_HEIGHT/2, "Awe dang...you dead!")
    }
}

// GameState is a trait from bracket_lib
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            Mode::Playing => self.playing_tick( ctx ),
            Mode::Dead => self.dead_tick( ctx )
        }
    }
}