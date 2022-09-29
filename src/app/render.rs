use std::io::{stdout, Write};

use crossterm::{terminal::{self, EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, Clear, ClearType}, style::{Color, SetForegroundColor, ResetColor}, execute, cursor::{MoveTo, position}, event::{PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags}};

use super::util::term_center;

pub struct Renderer;

#[derive(Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right
}

impl Renderer {
    pub fn init () -> Self {
        enable_raw_mode()
        .expect("This app requires raw mode to be available in order to function correctly");
        let mut stdout = stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        ).unwrap();
        Self
    }
    pub fn exit(self: &Self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            PopKeyboardEnhancementFlags,
            LeaveAlternateScreen,
        ).unwrap();
    
        disable_raw_mode().unwrap();
    }
    pub fn reset_color(self: &Self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            ResetColor,
        ).expect("Could not reset terminal colours");
    }
    pub fn print(self: &Self, word: &str, color: Option<Color>, background_color: Option<Color>) {
        let (row, col) = position().unwrap();
        self.print_at(
            word, 
            (row, col), 
            TextAlign::Left, 
            color, background_color, 
            Some(Clear(ClearType::CurrentLine))
        )
    }
    pub fn print_at_center_default(self: &Self, word: &str) {
        self.print_at(
            word, term_center(), 
            TextAlign::Center, 
            None, None, 
            Some(Clear(ClearType::CurrentLine))
        )
    }
    pub fn print_at_center(self: &Self, word: &str, offset: (i16, i16), align: Option<TextAlign>, color: Option<Color>, background_color: Option<Color>, clear: Option<Clear>) {
        let center = term_center();
        self.print_at(
            word, 
            (
                (center.0 as i16 + offset.0) as u16, 
                (center.1 as i16 + offset.1) as u16
            ), 
            align.unwrap_or(TextAlign::Center), 
            color, background_color, 
            clear
        )
    }
    pub fn print_at(self: &Self, word: &str, position: (u16, u16), align: TextAlign, color: Option<Color>, background_color: Option<Color>, clear: Option<Clear>) {
        let mut stdout = stdout();
        if let Some(color) = color {
            execute!(
                stdout,
                SetForegroundColor(color),
            ).expect("Could not set text colour");
        }
        if let Some(color) = background_color {
            execute!(
                stdout,
                SetForegroundColor(color),
            ).expect("Could not set background colour");
        }
        let x ;
        let y = position.1;
        match align {
            TextAlign::Left => {
                x = position.0;
            }
            TextAlign::Center => {
                x = position.0 - word.chars().count() as u16 / 2;
            }
            TextAlign::Right => {
                x = position.0 - word.chars().count() as u16;
            }
        }
        execute!(
            stdout,
            MoveTo(x, y),
        ).expect(format!("Could not move to position {} {}", x, y).as_str());
        if let Some(clear) = clear {
            execute!(
                stdout,
                clear,
            ).expect("Could not clear line");
        }
        write!(
            stdout,
            "{}", word,
        ).expect("Could not write to stdout");
        stdout.lock().flush().unwrap();
        execute!(
            stdout,
            ResetColor
        ).expect("Could not reset text colour after render");
    }
    pub fn clear_line_at_center(self: &Self, offset: (i16, i16)) {
        let center = term_center();
        self.clear_line_at(
            (
                (center.0 as i16 + offset.0) as u16, 
                (center.1 as i16 + offset.1) as u16
            ), 
        )
    }
    pub fn clear_line_at(self: &Self, position: (u16, u16)) {
        let mut stdout = stdout();
        execute!(
            stdout,
            MoveTo(position.0, position.1),
            Clear(ClearType::CurrentLine)
        ).expect("Could not clear line")
    }
}