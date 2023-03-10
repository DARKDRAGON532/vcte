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
    pub movable: bool
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            movable: true
        }
    }

    pub fn update_coords(&mut self, coords: (u16, u16), file: &mut File) {
        file.x = coords.0;
        file.y = coords.1;
    }

    pub fn move_to(&mut self, x: u16, y: u16, renderer: &mut Stdout, file: &mut File) -> Result<()> {
        queue!(renderer, cursor::MoveTo(x, y))?;
        self.update_coords((x, y), file);
        Ok(())
    }

    pub fn move_cursor(&mut self, direction: KeyCode, renderer: &mut Stdout, file: &mut File) -> Result<()> {
        if self.movable {
            let (terminal_x, terminal_y) = terminal::size()?;
            match direction {
                KeyCode::Up | KeyCode::Char('w') => {
                    if file.y == 2 && file.path != "[for init]" {
                        file.offset_y = file.offset_y.saturating_sub(1);
                    } 
                    else {
                        queue!(renderer, cursor::MoveUp(1))?;
                    }
                },
                KeyCode::Down | KeyCode::Char('s')=> {
                    if file.y == terminal_y-3 {
                        file.offset_y += 1;
                    } 
                    else {
                        queue!(renderer, cursor::MoveDown(1))?;
                    }
                },
                KeyCode::Left | KeyCode::Char('a') => {
                    if file.x == 0 {
                        file.offset_x = file.offset_x.saturating_sub(1)
                    } 
                    else {
                        queue!(renderer, cursor::MoveLeft(1))?;
                    }
                },
                KeyCode::Right | KeyCode::Char('d') => {
                    if file.x == terminal_x-1 {
                        file.offset_x += 1;
                    } 
                    else {
                        queue!(renderer, cursor::MoveRight(1))?;
                    }
                },
                _ => {}
            }
            self.update_coords(cursor::position()?, file);
        }
        Ok(())
    }
}