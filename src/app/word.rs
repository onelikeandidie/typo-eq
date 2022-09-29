use chrono::Utc;

pub struct Word {
    pub size: usize,
    pub original: String,
    pub translation: String,
}

pub struct State {
    pub progress: usize,
    pub failed: bool,
    pub wpm: f64,
    pub started_at: i64,
    pub last_word_timestamp: i64,
    pub stats: Stats,
}

impl Default for State {
    fn default() -> Self {
        let current_time = Utc::now().timestamp_millis();
        Self {
            progress: 0,
            failed: false,
            wpm: 0.0,
            started_at: current_time,
            last_word_timestamp: current_time,
            stats: Stats::default(),
        }
    }
}

#[derive(Default)]
pub struct Stats {
    pub completed: u64,
    pub chars_typed: u64,
    pub chars_failed: u64,
}