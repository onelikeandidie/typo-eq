use std::{fs::File, io::{SeekFrom, BufReader, Seek}};

use crate::importer::dictionary::{DictionaryWord, DictionaryPhrase, DictionaryEntry, Dictionary};

use super::base::DictionaryParser;

pub struct TypoEQParser;

impl DictionaryParser for TypoEQParser {
    fn parse(self: &Self, mut file: File) -> crate::importer::dictionary::Dictionary {
        file.seek(SeekFrom::Start(0)).unwrap();
        let file = BufReader::new(file);
        let mut entries: Vec<DictionaryEntry> = Vec::new();
        let mut words: Vec<DictionaryWord> = Vec::new();
        let mut phrases: Vec<DictionaryPhrase> = Vec::new();
        let mut word: Option<DictionaryWord> = None;
        let mut phrase: Option<DictionaryPhrase> = None;
        let mut buf = String::new();
        let mut from = "Unknown".to_string();
        let mut to = "Unkown".to_string();
        todo!();
        return Dictionary { entries, words, phrases, from, to };
    }
}