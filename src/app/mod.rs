use std::fs::File;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;
use crossterm::cursor::{MoveLeft, MoveToColumn, MoveToPreviousLine, MoveToNextLine};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{SetForegroundColor, Color, ResetColor};
use crossterm::{self, execute, ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;
use chrono::offset::Utc;

pub mod events;
pub mod word;
pub mod render;
pub mod cursor;
pub mod util;

use crate::app::cursor::Cursor;
use crate::app::events::*;
use crate::app::render::{Renderer, TextAlign};
use crate::app::word::*;

use crate::{config::Config, importer::dictionary::Dictionary};

pub const SKIP_CHARACTERS: [char; 2] = [
    '/', '|'
];

fn zero() -> (u16, u16) {(0, 0)}

pub fn create_app(config: Config) {
    let mut stdout = stdout();

    let renderer = Renderer::init();

    let (ltx, lrx) = mpsc::channel::<AppEvent>();
    let _loading_thread = thread::spawn(move || {
        ltx.send(AppEvent::LoadingStarted).unwrap();
        let file = File::open(config.dictionary_path.clone())
            .expect("Could not load the dictionary file");
        let dict = Dictionary::from_file(file);
        ltx.send(AppEvent::DictionaryLoaded(dict)).unwrap();
        ltx.send(AppEvent::LoadingFinished).unwrap();
    });

    let mut dict: Option<Dictionary> = None;
    while let Ok(event) = lrx.recv() {
        match event {
            AppEvent::LoadingStarted => {
                renderer.print_at_center_default("Loading Started");
            }
            AppEvent::DictionaryLoaded(loaded_dict) => {
                dict = Some(loaded_dict);
                renderer.print_at_center_default("Dictionary Loaded");
            }
            AppEvent::LoadingFinished => {
                renderer.print_at_center_default("Finished Loading");
            }
            _ => {},
        }
    }
    let dict = dict.expect("Could not load dictionary");
    
    let mut state = State::default();

    sleep(Duration::from_secs(1));
    let mut word = new_word(&dict);
    renderer.print_at_center_default(format!(
        "{} -> {}", 
        word.original, 
        word.translation
    ).as_str());

    while let Ok(event) = read() {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, 
                ..
            }) | Event::Key(KeyEvent {
                code: KeyCode::Char('c'), 
                modifiers: KeyModifiers::CONTROL, 
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => {
                if state.progress == 0 {
                    continue;
                }
                state.progress -= 1;
                state.failed = false;
                stdout.execute(MoveLeft(1)).unwrap();
                print!("  ");
                stdout.execute(MoveToColumn(state.progress as u16)).unwrap();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c), 
                ..
            }) => {
                let chars: Vec<char> = word.original.chars().collect::<Vec<char>>();
                let current_char: Option<&char> = chars
                    .get(state.progress);
                if let Some(current_char) = current_char {
                    // Check if the character is a skippable one and skip if 
                    // the user pressed any non letter or number keys
                    if (SKIP_CHARACTERS.contains(current_char) && !c.is_alphanumeric())
                    // Progress if the character input was correct
                    || current_char == &c {
                        if state.failed {
                            // Move cursor back and write the right char
                            stdout.execute(MoveLeft(1)).unwrap();
                        }
                        renderer.print_at_center(
                            format!("{}", c).as_str(),
                            (state.progress as i16 - (word.size / 2) as i16, 2),
                            None, None, None,
                            None
                        );
                        state.progress += 1;
                        state.failed = false;
                    } else {
                        if !state.failed {
                            renderer.print_at_center(
                                format!("{}", current_char).as_str(),
                                (state.progress as i16 - (word.size / 2) as i16, 2), 
                                None, Some(Color::DarkRed), None,
                                None
                            );
                        }
                        state.failed = true;
                        state.stats.chars_failed += 1;
                    }
                    state.stats.chars_typed += 1;
                    stdout.lock().flush().unwrap();
                }
                // Update progress display
                renderer.print_at_center(
                    format!("{}/{}", state.progress, word.size).as_str(),
                    (6, -2), Some(TextAlign::Left), 
                    Some(Color::DarkGrey), None,
                    None
                );
                // Update wpm display
                let current_timestamp = Utc::now().timestamp_millis();
                let diff = current_timestamp - state.last_word_timestamp;
                state.wpm = 1.0 / (diff as f64 / 1000.0 / 60.0);
                renderer.print_at_center(
                    format!("{} wpm", state.wpm.round()).as_str(), 
                    (-6, -2), Some(TextAlign::Right), 
                    Some(Color::DarkYellow), None,
                    None
                );
                Cursor::move_to_center(((state.progress as i16 - (word.size / 2) as i16), 2));
                if state.progress >= word.size {
                    // Update last word completed timestamp
                    state.last_word_timestamp = Utc::now().timestamp_millis();
                    state.stats.completed += 1;
                    // Show the completed word in grey
                    renderer.print_at_center(
                        format!(
                            "{} -> {}", 
                            word.original, 
                            word.translation
                        ).as_str(), (0, -4),
                        None, Some(Color::DarkGrey), None,
                        Some(Clear(ClearType::CurrentLine)),
                    );
                    // Clear the user input to output the right word in case
                    // there was any rendering mistake
                    renderer.clear_line_at_center((0, 2));
                    // execute!(stdout,
                    //     SetForegroundColor(Color::DarkGreen),
                    //     MoveToNextLine(1),
                    //     Clear(ClearType::CurrentLine),
                    //     MoveToColumn(0),
                    // ).unwrap();
                    // println!("{}", word.original);
                    // New word
                    word = new_word(&dict);
                    state.progress = 0;
                    renderer.print_at_center_default(format!(
                        "{} -> {}", 
                        word.original, 
                        word.translation
                    ).as_str());
                    Cursor::move_to_center((0, 2));
                }
            }
            _ => {},
        }
    }

    println!();
    // Update wpm
    let current_timestamp = Utc::now().timestamp_millis();
    let diff = current_timestamp - state.started_at;
    state.wpm = state.stats.completed as f64 / (diff as f64 / 1000.0 / 60.0);
    let out = format!(
        "Completed: {} words. {} chars typed, of which {} were misses ({}% Accuracy). Average wpm: {}",
        state.stats.completed, state.stats.chars_typed, state.stats.chars_failed,
        100.0 - (state.stats.chars_failed as f64 / state.stats.chars_typed as f64 * 100.0).round(),
        state.wpm.round(),
    );
    renderer.print_at_center(
        out.as_str(), (0, -2),
        None, Some(Color::DarkYellow), None,
        Some(Clear(ClearType::CurrentLine))
    );
    Cursor::move_to_center((0, 8));
}

pub fn new_word(dict: &Dictionary) -> Word {
    let mut rng = thread_rng();
    let distribuition = Uniform::new(0, dict.entries.len());
    let word_index = distribuition.sample(&mut rng);
    let word = dict.entries.get(word_index);
    if let Some(word) = word {
        return Word {
            size: word.identifier.chars().count(),
            original: word.identifier.clone(),
            translation: word.translation.clone(),
        }
    }
    panic!("Word could not be selected, out of bounds");
}