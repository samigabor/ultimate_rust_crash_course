use std::io::Stdout;
use crate::frame::Frame;
use crossterm::terminal::{Clear, ClearType};
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::QueueableCommand;
use std::io::Write;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, c) in col.iter().enumerate() {
            if *c != last_frame[x][y] || force {
                stdout.queue(crossterm::cursor::MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", c);
            }
        }
    }
    stdout.flush().unwrap();
}