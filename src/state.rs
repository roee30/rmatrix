use crate::env::Env;
use crate::flake::Flake;
use crossterm::{
    cursor::{Hide, RestorePosition, SavePosition, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
    Result,
};
use std::{io::stdout, thread, time};

pub struct State {
    flakes: Vec<Flake>,
    env: Env,
    cleanup: Cleanup,
}

impl State {
    pub fn make(env: Env, flakes: Vec<Flake>) -> Self {
        Self {
            env: env,
            flakes: flakes,
            cleanup: Cleanup { cleaned: false },
        }
    }
    fn render(&self) -> Result<()> {
        self.flakes.iter().map(Flake::render).collect()
    }

    fn next(self) -> State {
        let mut env = self.env;
        let rows = env.size.rows;
        let mut fs: Vec<Flake> = self
            .flakes
            .into_iter()
            .filter(|f| f.start < rows)
            .map(|f| f.next(&mut env))
            .collect();
        if fs.len() < env.max_flakes as usize {
            fs.push(Flake {
                start: 0,
                column: (&mut env).new_flake_column(),
                content: vec![env.random_char()],
            });
        }
        Self {
            env: env,
            flakes: fs,
            cleanup: self.cleanup,
        }
    }
    pub fn run(self) -> Result<()> {
        execute!(stdout(), terminal::EnterAlternateScreen, Hide, SavePosition)?;
        enable_raw_mode()?;
        let mut s = self;
        loop {
            s.render()?;
            if poll(time::Duration::from_secs(0))? {
                if let Event::Key(e) = read()? {
                    match e {
                        KeyEvent {
                            code: KeyCode::Char(c @ '0'..='9'),
                            ..
                        } => {
                            let num = c.to_digit(10).unwrap() as u16;
                            s.env.speed = if num == 0 { 10 } else { num }
                        }
                        KeyEvent {
                            code: KeyCode::Char('x'),
                            ..
                        } => {
                            s.cleanup.cleanup();
                            return Ok(());
                        }
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: m,
                        } if m.contains(KeyModifiers::CONTROL) => {
                            s.cleanup.cleanup();
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            s = s.next();

            thread::sleep(time::Duration::from_secs_f64(
                (11 - s.env.speed) as f64 * s.env.delay_base,
            ));
        }
    }
}
struct Cleanup {
    cleaned: bool,
}

impl Cleanup {
    fn cleanup(&mut self) {
        if !self.cleaned {
            let _ = disable_raw_mode();
            let _ = execute!(
                stdout(),
                terminal::LeaveAlternateScreen,
                Show,
                RestorePosition
            );
            self.cleaned = true;
        }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        self.cleanup();
    }
}
