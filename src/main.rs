mod env;
mod flake;
mod state;

use crossterm::{terminal::size, Result};
use env::{Env, Size};
use state::State;

fn main() -> Result<()> {
    let (cols, rows) = size()?;
    State::make(Env::make(Size { rows, cols }), Vec::new()).run()
}
