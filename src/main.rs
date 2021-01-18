use crossterm::{
    cursor::{Hide, MoveTo, Show, SavePosition, RestorePosition},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
    Result,
};
use rand::{rngs::ThreadRng, Rng};
use std::io::stdout;
use std::{thread, time};

struct Size {
    rows: u16,
    cols: u16,
}
type Age = u16;

struct Env {
    rng: ThreadRng,
    size: Size,
    max_flakes: u16,
    last_generated: Age,
    flake_gap: Age,
    max_length: u16,
    delay_base: f64,
    speed: u16,
}

impl Env {
    fn make(size: Size) -> Self {
        Self {
            size: size,
            max_flakes: 70,
            last_generated: 0,
            flake_gap: 0,
            rng: rand::thread_rng(),
            max_length: 10,
            speed: 10,
            delay_base: 0.05,
        }
    }

    fn new_flake_column(&mut self) -> u16 {
        self.rng.gen_range(0..self.size.cols)
    }

    fn random_char(&mut self) -> char {
        // char::from(self.rng.sample(Alphanumeric))
        std::char::from_u32(self.rng.gen_range(0xff65..=0xff9f)).unwrap()
    }
}

struct Flake {
    start: u16,
    column: u16,
    str: Vec<char>,
}

fn print_at(x: u16, y: u16, color: Color, str: char) -> Result<()> {
    execute!(
        stdout(),
        MoveTo(x, y),
        SetForegroundColor(color),
        SetAttribute(Attribute::Bold),
        Print(str),
        // ResetColor,
    )?;
    Ok(())
}

fn push<T: Copy>(vec: &Vec<T>, item: T) -> Vec<T> {
    let mut new: Vec<T> = vec.iter().map(|c| *c).collect();
    new.push(item);
    new
}

impl Flake {
    fn render(&self) -> Result<()> {
        let head = self.start + self.str.len() as u16 - 1;
        print_at(
            self.column,
            head,
            Color::White,
            *self.str.iter().last().unwrap(),
        )?;
        if head > 0 {
            print_at(
                self.column,
                head - 1,
                Color::Green,
                *self.str.iter().last().unwrap(),
            )?;
        }
        print_at(self.column, self.start, Color::White, ' ')?;
        Ok(())
    }
    fn next(&self, env: &mut Env) -> Self {
        let (start, base) = if self.str.len() < env.max_length as usize {
            (self.start, self.str.clone())
        } else {
            (
                self.start + 1,
                self.str.iter().skip(1).map(|c| *c).collect(),
            )
        };
        Self {
            start: start,
            column: self.column,
            str: push(&base, env.random_char()),
        }
    }
}

struct State {
    flakes: Vec<Flake>,
    env: Env,
    age: Age,
    cleanup: Cleanup,
}

impl State {
    fn make(env: Env, flakes: Vec<Flake>) -> Self {
        Self {
            env: env,
            flakes: flakes,
            age: 0,
            cleanup: Cleanup { cleaned: false },
        }
    }
    fn render(&self) -> Result<()> {
        for f in &self.flakes {
            f.render()?;
        }
        Ok(())
    }

    fn next(self) -> State {
        let mut env = self.env;
        let rows = env.size.rows;
        let mut fs: Vec<Flake> = self
            .flakes
            .iter()
            .filter(|f| f.start < rows)
            .map(|f| f.next(&mut env))
            .collect();
        if fs.len() < env.max_flakes as usize && self.age - env.last_generated >= env.flake_gap {
            fs.push(Flake {
                start: 0,
                column: (&mut env).new_flake_column(),
                str: vec![env.random_char()],
            });
            env.last_generated = self.age;
        }
        Self {
            env: env,
            flakes: fs,
            age: self.age + 1,
            cleanup: self.cleanup,
        }
    }
    fn run(self) -> Result<()> {
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
        }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        self.cleanup();
    }
}


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
