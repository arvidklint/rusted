use crossterm::event::KeyCode;

use crate::cursor::Cursor;
use crate::line::Line;
use crate::math::clamp;
use crate::renderer::Renderer;

pub struct Buffer {
    lines: Vec<Line>,
    cursor: Cursor,
}

impl Buffer {
    pub fn new(body: &str) -> Self {
        let cursor = Cursor::new();
        let lines = body.lines().map(|l| Line::new(l)).collect();

        Self { lines, cursor }
    }

    fn insert_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
            return;
        }

        let line = if let Some(line) = self.lines.get_mut(self.cursor.row()) {
            line
        } else {
            return;
        };

        line.insert(self.cursor.col(), c);
        self.move_cursor(0, 1);
    }

    fn delete_backwards(&mut self) {
        if self.cursor.col() <= 0 && self.cursor.row() <= 0 {
            return;
        }

        let line = if let Some(line) = self.lines.get_mut(self.cursor.row()) {
            line
        } else {
            return;
        };

        if self.cursor.col() <= 0 {
            // append current line to the one above
            let mut characters = line.split_off(0);
            self.lines.remove(self.cursor.row());
            self.move_cursor(-1, 0);
            let line = if let Some(line) = self.lines.get_mut(self.cursor.row()) {
                line
            } else {
                return;
            };
            let line_len = line.len();
            line.append(&mut characters);
            self.set_cursor(self.cursor.row() as isize, line_len as isize);
            return;
        }

        line.delete(self.cursor.col() - 1);
        self.move_cursor(0, -1);
    }

    pub fn new_line(&mut self) {
        let lines_len = self.lines.len();

        let line = if let Some(line) = self.lines.get_mut(self.cursor.row()) {
            line
        } else {
            return;
        };
        let line_len = line.len();

        if self.cursor.col() >= line_len && self.cursor.row() + 1 >= lines_len {
            // at end of line on last line. push new line
            self.lines.push(Line::new(""));
            self.move_cursor(1, 0);
            return;
        }

        if self.cursor.row() + 1 >= lines_len {
            // on last line, insert new line below and move cursor down
            let rest_of_characters = line.split_off(self.cursor.col());
            self.lines
                .push(Line::from_character_list(rest_of_characters));
            self.set_cursor(self.cursor.row() as isize + 1, 0);
            return;
        }

        // get new line from characters after cursor
        let rest_of_characters = line.split_off(self.cursor.col());
        self.set_cursor(self.cursor.row() as isize + 1, 0);
        self.lines.insert(
            self.cursor.row(),
            Line::from_character_list(rest_of_characters),
        );
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        match renderer.render(&self.get_body()) {
            _ => (),
        }

        match renderer.set_cursor(self.cursor.row(), self.cursor.col()) {
            _ => (),
        }
    }

    pub fn process_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Backspace => self.delete_backwards(),
            KeyCode::Left => self.move_cursor(0, -1),
            KeyCode::Right => self.move_cursor(0, 1),
            KeyCode::Up => self.move_cursor(-1, 0),
            KeyCode::Down => self.move_cursor(1, 0),
            KeyCode::Enter => self.new_line(),
            _ => {}
        }
    }

    fn get_body(&self) -> String {
        self.lines
            .iter()
            .map(|line| format!("{}\n", line.get_string()))
            .collect()
    }

    fn set_cursor(&mut self, row: isize, col: isize) {
        let row = clamp(row, 0, self.lines.len() as isize - 1);

        let col = if let Some(line) = self.lines.get(row as usize) {
            clamp(col, 0, line.len() as isize)
        } else {
            0
        };

        self.cursor.set_position(row, col);
    }

    fn move_cursor(&mut self, delta_row: isize, delta_col: isize) {
        // calculate new positions
        let row = self.cursor.row() as isize + delta_row;
        let col = self.cursor.col() as isize + delta_col;

        self.set_cursor(row, col);
    }
}
