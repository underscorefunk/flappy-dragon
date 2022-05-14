// Register modules so that they can be referenced by
// `use crate::my_module_name` i.e. `use crate::player`

// v-- Registrations of modules in this crate
mod player;
mod score;
mod state;
mod window;
mod obstacle;

// v-- Importing modules for use in this *.rs file
use ::bracket_lib::prelude::*;
use crate::state::State;
// ^- you could write this as `crate::state::State` to import
// the public struct `State` from the `state` module of this crate
// or you could write `use state::State` or `use state::*` to import
// everything.

// By splitting things up in to modules the main function gets
// a lot more clear
fn main() -> BError {

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(
        context,
        State::new(),
    )
}
