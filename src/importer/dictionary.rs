use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum DictionaryEntry {
    Word(DictionaryWord),
    Phrase(DictionaryPhrase),
}

#[derive(Debug, Clone)]
pub struct DictionaryWord {
    pub kind: String,
    pub identifier: String,
    pub translation: Vec<String>,
}

impl DictionaryWord {
    pub fn new(kind: String) -> Self {
        Self {
            kind,
            identifier: String::new(),
            translation: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DictionaryPhrase {
    pub kind: String,
    pub identifier: String,
    pub translation: String,
    pub example_for: String,
}

impl DictionaryPhrase {
    pub fn new(kind: String) -> Self {
        Self {
            kind,
            identifier: String::new(),
            translation: String::new(),
            example_for: String::new(),
        }
    }
}

impl Display for DictionaryWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.identifier, self.translation.concat())
    }
}

#[derive(Clone)]
pub struct Dictionary {
    pub entries: Vec<DictionaryEntry>,
    pub words: Vec<DictionaryWord>,
    pub phrases: Vec<DictionaryPhrase>,
    pub from: String,
    pub to: String,
}
