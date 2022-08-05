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
    pub str: Vec<char>,
}

impl Flake {
    pub fn render(&self) -> Result<()> {
        if self.str.len() == 0 {
            return Ok(());
        }
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
                // *self.str.iter().last().unwrap(),
                self.str[self.str.len() - 2]
            )?;
        }
        print_at(self.column, self.start, Color::White, ' ')?;
        Ok(())
    }
    pub fn next(&self, env: &mut Env) -> Self {
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
