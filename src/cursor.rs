use std::io::Stdout;

use crossterm::{
    cursor,
    event::KeyCode,
    queue,
    Result,
    terminal
};

use crate::window::File;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    pub movable: bool
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            movable: true
        }
    }

    pub fn update_coords(&mut self, coords: (u16, u16)) {
        self.x = coords.0;
        self.y = coords.1;
    }

    pub fn move_to(&mut self, x: u16, y: u16, renderer: &mut Stdout) -> Result<()> {
        queue!(renderer, cursor::MoveTo(x, y))?;
        self.update_coords((x, y));
        Ok(())
    }

    pub fn move_cursor(&mut self, direction: KeyCode, renderer: &mut Stdout, file: &mut File) -> Result<()> {
        if self.movable {
            let (terminal_x, terminal_y) = terminal::size()?;
            match direction {
                KeyCode::Up | KeyCode::Char('w') => {
                    if self.y == 2 {
                        file.offset_y = file.offset_y.saturating_sub(1);
                    } 
                    else {
                        queue!(renderer, cursor::MoveUp(1))?;
                    }
                },
                KeyCode::Down | KeyCode::Char('s')=> {
                    if self.y == terminal_y-2 {
                        file.offset_y += 1;
                    } 
                    else {
                        queue!(renderer, cursor::MoveDown(1))?;
                    }
                },
                KeyCode::Left | KeyCode::Char('a') => {
                    if self.x == 0 {
                        file.offset_x = file.offset_x.saturating_sub(1)
                    } 
                    else {
                        queue!(renderer, cursor::MoveLeft(1))?;
                    }
                },
                KeyCode::Right | KeyCode::Char('d') => {
                    if self.x == terminal_x-1 {
                        file.offset_x += 1;
                    } 
                    else {
                        queue!(renderer, cursor::MoveRight(1))?;
                    }
                },
                _ => {}
            }
            self.update_coords(cursor::position()?);
        }
        Ok(())
    }
}