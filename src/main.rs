mod env;
mod flake;
mod state;

use crossterm::{terminal::size, Result};
use env::{Env, Size};
use flake::Flake;
use state::State;

fn main() -> Result<()> {
    let (cols, rows) = size()?;
    let mut env = Env::make(Size { rows, cols });
    let initial = env.new_flake_column();
    let init_char = env.random_char();

    State::make(
        env,
        vec![Flake {
            start: 0,
            column: initial,
            str: vec![init_char],
        }],
    )
    .run()
}
