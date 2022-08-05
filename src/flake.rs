use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
    Result,
};
use std::io::stdout;

use crate::env::Env;

#[derive(Debug)]
pub struct Flake {
    pub start: u16,
    pub column: u16,
    pub content: Vec<char>,
}

impl Flake {
    pub fn render(&self) -> Result<()> {
        if self.content.len() == 0 {
            return Ok(());
        }
        let head = self.start + self.content.len() as u16 - 1;
        print_at(
            self.column,
            head,
            Color::White,
            *self.content.iter().last().unwrap(),
        )?;
        if head > 0 {
            print_at(
                self.column,
                head - 1,
                // Color::Green results in grey for some reason
                Color::AnsiValue(106),
                self.content[self.content.len() - 2]
            )?;
        }
        print_at(self.column, self.start, Color::White, ' ')
    }
    pub fn next(self, env: &mut Env) -> Self {
        let (start, base) = if self.content.len() < env.max_length as usize {
            (self.start, self.content.clone())
        } else {
            (
                self.start + 1,
                self.content.iter().skip(1).map(|c| *c).collect(),
            )
        };
        Self {
            start: start,
            column: self.column,
            content: push(base, env.random_char()),
        }
    }
}
fn print_at(x: u16, y: u16, color: Color, str: char) -> Result<()> {
    execute!(
        stdout(),
        SetAttribute(Attribute::Reset),
        MoveTo(x, y),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(color),
        Print(str),
        SetAttribute(Attribute::Reset),
    )
}
fn push<T>(mut vec: Vec<T>, item: T) -> Vec<T> {
    vec.push(item);
    vec
}
