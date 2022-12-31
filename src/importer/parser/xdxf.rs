use std::{fs::File, io::{SeekFrom, BufReader, Seek}};

use quick_xml::{Reader, events::Event};

use crate::importer::dictionary::{DictionaryWord, DictionaryPhrase, DictionaryEntry, Dictionary};

use super::base::DictionaryParser;

pub struct XDXFParser;

impl DictionaryParser for XDXFParser {
    fn parse(self: &Self, mut file: File) -> crate::importer::dictionary::Dictionary {
        file.seek(SeekFrom::Start(0)).unwrap();
        let file = BufReader::new(file);
        let mut entries = Vec::new();
        let mut words = Vec::new();
        let mut phrases = Vec::new();
        let mut from = "Unkown".to_string();
        let mut to   = "Unkown".to_string();

        let mut parser = Reader::from_reader(file);
        let mut word = None;
        let mut phrase = None;
        let mut buf = Vec::new();
        let mut current_tag = String::new();
        let mut current_attrs = Vec::new();
        loop {
            match parser.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", parser.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let tag = e.name();
                    let tag = String::from_utf8(tag.as_ref().to_vec()).unwrap();
                    if tag.as_str() == "ar" {
                        let new_entry = DictionaryWord::new(tag.clone());
                        word = Some(new_entry);
                    }
                    if tag.as_str() == "xdxf" {
                        let lang_from = e.try_get_attribute("lang_from");
                        let lang_to = e.try_get_attribute("lang_to");
                        if let Ok(lang_from) = lang_from {
                            if let Some(lang_from) = lang_from {
                                from = String::from_utf8(lang_from.value.to_vec()).unwrap();
                            }
                        }
                        if let Ok(lang_to) = lang_to {
                            if let Some(lang_to) = lang_to {
                                to = String::from_utf8(lang_to.value.to_vec()).unwrap();
                            }
                        }
                    }
                    if tag.as_str() == "exm" {
                        let new_phrase = DictionaryPhrase::new(tag.clone());
                        phrase = Some(new_phrase);
                    }
                    let attrs = e.attributes().map(|attr| {
                        if let Ok(attr) = attr {
                            String::from_utf8(attr.value.to_vec()).unwrap()
                        } else {
                            String::new()
                        }
                    });
                    current_tag = tag;
                    current_attrs = attrs.collect();
                }
                Ok(Event::Text(e)) => {
                    if let Some(entry) = word.as_mut() {
                        if current_tag == "k" {
                            entry.identifier = e.unescape().unwrap().to_string();
                            if let Some(phrase) = phrase.as_mut() {
                                phrase.example_for = entry.identifier.clone();
                            }
                        }
                        if current_tag == "dtrn" {
                            entry.translation.push(e.unescape().unwrap().to_string());
                        }
                        if current_tag == "ex" && current_attrs.contains(&"exm".to_string()) {
                            if let Some(phrase) = phrase.as_mut() {
                                phrase.identifier = e.unescape().unwrap().to_string();
                            }
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    let tag = e.name();
                    let tag = String::from_utf8(tag.as_ref().to_vec()).unwrap();
                    if tag.as_str() == "ar" {
                        if let Some(new_word) = word {
                            words.push(new_word.clone());
                            entries.push(DictionaryEntry::Word(new_word));
                            word = None;
                        }
                        if let Some(new_phrase) = phrase {
                            phrases.push(new_phrase.clone());
                            entries.push(DictionaryEntry::Phrase(new_phrase));
                            phrase = None;
                        }
                    }
                    current_tag = String::new();
                    current_attrs = Vec::new();
                }
                _ => {}
            }
            buf.clear();
        }
        return Dictionary { entries, words, phrases, from, to };
    }
}