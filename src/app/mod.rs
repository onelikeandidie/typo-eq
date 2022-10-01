use std::fs::File;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Color;
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

pub fn create_app(config: Config) {
    let stdout = stdout();

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
    let mut load_time = 0;
    while let Ok(event) = lrx.recv() {
        match event {
            AppEvent::LoadingStarted => {
                load_time = Utc::now().timestamp_millis();
                renderer.print_at_center_default("Loading Started");
            }
            AppEvent::DictionaryLoaded(loaded_dict) => {
                dict = Some(loaded_dict);
                renderer.print_at_center_default("Dictionary Loaded");
            }
            AppEvent::LoadingFinished => {
                renderer.print_at_center_default( format!(
                    "Finished Loading ({} sec)",
                    (Utc::now().timestamp_millis() - load_time) as f64 / 1000.0
                ).as_str());
            }
        }
    }
    let dict = dict.expect("Could not load dictionary");
    
    let mut state = State::default();

    sleep(Duration::from_millis(500));
    // Show dictionary loaded
    renderer.print_at_center(
        format!("{} -> {}", dict.from, dict.to).as_str(),
        (0, -6), None, None, None, None,
    );
    let mut old_words: Vec<Word> = Vec::new();
    let mut word = new_word(&dict);
    renderer.clear_line_at_center((0,0));
    render_translations(&renderer, &word);
    render_center(&renderer, &word, &state);
    render_cursor(&renderer, &word, &state);

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
                state.failed = false;
                render_center(&renderer, &word, &state);
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
                        state.progress += 1;
                        state.failed = false;
                    } else {
                        state.failed = true;
                        state.stats.chars_failed += 1;
                    }
                    state.stats.chars_typed += 1;
                    stdout.lock().flush().unwrap();
                }
                let current_timestamp = Utc::now().timestamp_millis();
                let diff = current_timestamp - state.last_word_timestamp;
                state.wpm = 1.0 / (diff as f64 / 1000.0 / 60.0);
                render_center(&renderer, &word, &state);
                if state.progress >= word.size {
                    // Update last word completed timestamp
                    state.last_word_timestamp = Utc::now().timestamp_millis();
                    state.stats.completed += 1;
                    // Add last word to the book of words
                    old_words.push(word);
                    if old_words.len() >= 5 {
                        old_words.remove(0);
                    }
                    render_completed_words(&renderer, &old_words);
                    // Clear the user input
                    renderer.clear_line_at_center((0, 2));
                    // New word
                    word = new_word(&dict);
                    state.progress = 0;
                    renderer.clear_line_at_center((0,0));
                    render_center(&renderer, &word, &state);
                    render_translations(&renderer, &word);
                }
            }
            _ => {},
        }
        render_cursor(&renderer, &word, &state);
    }
    // Update wpm
    let current_timestamp = Utc::now().timestamp_millis();
    let diff = current_timestamp - state.started_at;
    state.wpm = state.stats.completed as f64 / (diff as f64 / 1000.0 / 60.0);
    // Render final screen
    let out1 = format!(
        "Completed: {} words. Average wpm: {}",
        state.stats.completed, 
        state.wpm.round(),
    );
    let out2 = format!(
        "{} chars typed, of which {} were misses ({}% Accuracy).",
        state.stats.chars_typed, state.stats.chars_failed,
        100.0 - (state.stats.chars_failed as f64 / state.stats.chars_typed as f64 * 100.0).round(),
    );
    renderer.print_at_center(
        out1.as_str(), (0, 2),
        None, Some(Color::DarkYellow), None,
        Some(Clear(ClearType::CurrentLine))
    );
    renderer.print_at_center(
        out2.as_str(), (0, 3),
        None, Some(Color::DarkYellow), None,
        Some(Clear(ClearType::CurrentLine))
    );
    // Move cursor out of frame as to continue out of raw mode [rp[[er]]]
    Cursor::move_to_center((0, 8));
}

pub fn render_center(renderer: &Renderer, word: &Word, state: &State) {
    // Update progress display
    renderer.print_at_center(
        format!("{}/{}", state.progress, word.size).as_str(),
        (word.size as i16 / 2 + 4, 0), Some(TextAlign::Left), 
        Some(Color::DarkGrey), None,
        None
    );
    // Update wpm display
    renderer.print_at_center(
        format!("{} wpm", state.wpm.round()).as_str(), 
        (- (word.size as i16 / 2) - 4, 0), Some(TextAlign::Right), 
        Some(Color::DarkYellow), None,
        None
    );
    // Update word shown
    let left  = word.original_chars[..state.progress].into_iter().collect::<String>();
    let right = word.original_chars[state.progress..].into_iter().collect::<String>();
    let fail_char = word.original_chars.get(state.progress).unwrap_or(&' ');
    let left_x   = - (word.size as i16 / 2);
    let right_x  = left_x + state.progress as i16;
    renderer.print_at_center(
        left.as_str(),
        (left_x, 0), Some(TextAlign::Left),
        Some(Color::DarkGreen), None, None
    );
    renderer.print_at_center(
        right.as_str(),
        (right_x, 0), Some(TextAlign::Left),
        None, None, None
    );
    if state.failed {
        renderer.print_at_center(
            format!("{}", fail_char).as_str(),
            (right_x, 0), Some(TextAlign::Left),
            Some(Color::DarkRed), None, None
        );
    }
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
            original_chars: word.identifier.chars().collect(),
            translation: word.translation.clone(),
        }
    }
    panic!("Word could not be selected, out of bounds");
}

pub fn render_completed_words(renderer: &Renderer, words: &Vec<Word>) {
    for i in 0..(words.len()) {
        let word = words.get((words.len() - 1) - i).unwrap();
        renderer.clear_line_at_center((0, -2 - i as i16));
        // Show the completed word in grey
        renderer.print_at_center(
            format!("{}", word.original).as_str(), 
            (-2, -2 - i as i16),
            Some(TextAlign::Right), Some(Color::DarkGrey), None,
            None,
        );
        renderer.print_at_center(
            "->", 
            (0, -2 - i as i16),
            None, Some(Color::DarkGrey), None,
            None,
        );
        renderer.print_at_center(
            format!(
                "{}", 
                word.translation.first().unwrap_or(&"no translation".to_string())
            ).as_str(), 
            (2, -2 - i as i16),
            Some(TextAlign::Left), Some(Color::DarkGrey), None,
            None,
        );
    }
}

pub fn render_translations(renderer: &Renderer, word: &Word) {
    renderer.clear_down_from_center_at(2);
    for i in 0..(word.translation.len()) {
        let translation = word.translation.get(i).unwrap();
        // Show the completed word in grey
        renderer.print_at_center(
            format!(
                "{}", 
                translation
            ).as_str(), (0, 2 + i as i16),
            None, None, None,
            None,
        );
    }
}

pub fn render_new_word(renderer: &Renderer, word: &Word) {
    renderer.print_at_center(
        format!(
            "{}", 
            word.original,
        ).as_str(), (0,0), None,
        None, None,
        Some(Clear(ClearType::CurrentLine))
    );
}

pub fn render_cursor(renderer: &Renderer, word: &Word, state: &State) {
    renderer.print_at_center(
        "^", (get_progress_cursor(word, state), 1), 
        None, 
        Some(Color::DarkYellow), None, 
        Some(Clear(ClearType::CurrentLine))
    );
}

pub fn get_progress_cursor(word: &Word, state: &State) -> i16 {
    state.progress as i16 - (word.size / 2) as i16
}

pub fn reset_cursor(word: &Word, state: &State) {
    let new_cursor_pos_x = get_progress_cursor(word, state) + (if state.failed {1} else {0});
    Cursor::move_to_center((new_cursor_pos_x, 0));
}