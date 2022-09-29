use std::io::stdout;

use crossterm::{execute, cursor::MoveTo};

use super::util::term_center;

pub struct Cursor;

impl Cursor {
    pub fn move_to_center_default() {
        let center = term_center();
        Self::move_to(
            center.0, center.1
        )
    }
    pub fn move_to_center(offset: (i16, i16)) {
        let center = term_center();
        Self::move_to(
            (center.0 as i16 + offset.0) as u16, 
            (center.1 as i16 + offset.1) as u16
        )
    }
    pub fn move_to(row: u16, col: u16) {
        let mut stdout = stdout();
        execute!(
            stdout,
            MoveTo(row, col)
        ).expect("Could not move cursor");
    }
}