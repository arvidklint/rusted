use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Stdout, Write};

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn render(&mut self, body: &String) -> crossterm::Result<()> {
        self.clear_screen()?;

        body.lines().enumerate().for_each(|(row, line)| {
            match queue!(
                self.stdout,
                cursor::SavePosition,
                cursor::MoveToRow(row as u16 + 1),
                cursor::MoveToColumn(0 + 1),
                SetForegroundColor(Color::Green),
                SetBackgroundColor(Color::Black),
                Print(line),
                cursor::RestorePosition,
                ResetColor,
            ) {
                Ok(_) => (),
                Err(_) => (),
            }
        });

        Ok(())
    }

    fn clear_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.stdout, Clear(ClearType::All))?;

        Ok(())
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) -> crossterm::Result<()> {
        queue!(
            self.stdout,
            cursor::MoveToRow(row as u16 + 1),
            cursor::MoveToColumn(col as u16 + 1),
        )?;

        Ok(())
    }

    pub fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush");
    }
}
