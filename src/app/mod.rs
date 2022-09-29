use std::fs::File;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread;
use crossterm::cursor::{MoveLeft, MoveToColumn, MoveToPreviousLine, MoveToNextLine};
use crossterm::event::{PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{SetForegroundColor, Color, ResetColor};
use crossterm::{self, execute, ExecutableCommand};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;
use chrono::offset::Utc;

pub mod events;
pub mod word;

use crate::app::events::*;
use crate::app::word::*;

use crate::{config::Config, importer::dictionary::Dictionary};

pub const SKIP_CHARACTERS: [char; 2] = [
    '/', '|'
];

pub fn create_app(config: Config) {
    enable_raw_mode()
        .expect("This app requires raw mode to be available in order to function correctly");
    
    let mut stdout = stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    ).unwrap();

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
            AppEvent::DictionaryLoaded(loaded_dict) => {
                dict = Some(loaded_dict);
            }
            _ => {},
        }
    }
    let dict = dict.expect("Could not load dictionary");
    
    let mut state = State::default();

    let mut word = new_word(&dict);
    println!("{} -> {}", word.original, word.translation);
    stdout.execute(MoveToColumn(0)).unwrap();

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
                        stdout.execute(MoveToColumn(state.progress as u16)).unwrap();
                        print!("{}", c);
                        state.progress += 1;
                        state.failed = false;
                    } else {
                        if !state.failed {
                            stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
                            print!("{}", current_char);
                            stdout.execute(ResetColor).unwrap();
                        }
                        state.failed = true;
                        state.stats.chars_failed += 1;
                    }
                    state.stats.chars_typed += 1;
                    stdout.lock().flush().unwrap();
                }
                // Update progress display
                execute!(
                    stdout,
                    MoveToColumn(word.size as u16 + 1),
                    SetForegroundColor(Color::DarkGrey),
                ).unwrap();
                print!("{}/{}", state.progress, word.size);
                // Update wpm display
                let current_timestamp = Utc::now().timestamp_millis();
                let diff = current_timestamp - state.last_word_timestamp;
                state.wpm = 1.0 / (diff as f64 / 1000.0 / 60.0);
                execute!(stdout,
                    MoveToColumn((word.size + word.translation.len() + 8).try_into().unwrap()),
                    SetForegroundColor(Color::DarkYellow),
                ).unwrap();
                print!("{} wpm", state.wpm.round());
                execute!(
                    stdout,
                    MoveToColumn(state.progress as u16),
                    ResetColor,
                ).unwrap();
                if state.progress >= word.size {
                    // Update last word completed timestamp
                    state.last_word_timestamp = Utc::now().timestamp_millis();
                    state.stats.completed += 1;
                    // Show the completed word in grey
                    execute!(stdout,
                        MoveToPreviousLine(1),
                        MoveToColumn(0),
                        SetForegroundColor(Color::DarkGrey),
                    ).unwrap();
                    println!("{} -> {}", word.original, word.translation);
                    // Clear the user input to output the right word in case
                    // there was any rendering mistake
                    execute!(stdout,
                        SetForegroundColor(Color::DarkGreen),
                        MoveToNextLine(1),
                        Clear(ClearType::CurrentLine),
                        MoveToColumn(0),
                    ).unwrap();
                    println!("{}", word.original);
                    // New word
                    word = new_word(&dict);
                    state.progress = 0;
                    execute!(stdout,
                        ResetColor,
                        MoveToColumn(0),
                    ).unwrap();
                    println!("{} -> {}", word.original, word.translation);
                    stdout.execute(MoveToColumn(0)).unwrap();
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
    println!(
        "Completed: {} words. {} chars typed, of which {} were misses ({}%). Average wpm: {}",
        state.stats.completed, state.stats.chars_typed, state.stats.chars_failed,
        (state.stats.chars_failed as f64 / state.stats.chars_typed as f64 * 100.0).round(),
        state.wpm.round(),
    );
    stdout.execute(MoveToColumn(0)).unwrap();
    
    execute!(
        stdout,
        PopKeyboardEnhancementFlags,
    ).unwrap();

    disable_raw_mode().unwrap();
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