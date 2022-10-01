use std::fmt::Display;
use std::io::{BufReader, SeekFrom};
use std::{fs::File, io::Seek};

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub kind: String,
    pub identifier: String,
    pub translation: Vec<String>,
}

impl DictionaryEntry {
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

impl Display for DictionaryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.identifier, self.translation.concat())
    }
}

#[derive(Clone)]
pub struct Dictionary {
    pub entries: Vec<DictionaryEntry>,
    pub from: String,
    pub to: String,
}

impl Dictionary {
    pub fn from_file(mut file: File) -> Self {
        file.seek(SeekFrom::Start(0)).unwrap();
        let file = BufReader::new(file);
        let mut dict = Vec::new();
        let mut phrases = Vec::new();
        let mut from = "Unkown".to_string();
        let mut to   = "Unkown".to_string();

        let mut parser = Reader::from_reader(file);
        let mut entry = None;
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
                        let new_entry = DictionaryEntry::new(tag.clone());
                        entry = Some(new_entry);
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
                    if let Some(entry) = entry.as_mut() {
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
                        if let Some(new_entry) = entry {
                            dict.push(new_entry);
                            entry = None;
                        }
                        if let Some(new_phrase) = phrase {
                            phrases.push(new_phrase);
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
        return Dictionary { entries: dict, from, to };
    }
}
