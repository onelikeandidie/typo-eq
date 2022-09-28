pub struct Word {
    pub size: usize,
    pub original: String,
    pub translation: String,
}

pub struct State {
    pub progress: usize,
    pub failed: bool,
    pub wpm: f64,
    pub last_word_timestamp: i64,
}