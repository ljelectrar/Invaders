use std::io::{Stdout, Write};
use crossterm::{cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};
use crate::frame::Frame;

pub fn render(Stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool){
    if force {
        Stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        Stdout.queue(Clear(ClearType::All)).unwrap();
        Stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                Stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    Stdout.flush().unwrap();
}   