use std::fmt::Display;
use std::io::{BufReader, SeekFrom};
use std::{fs::File, io::Seek};

use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub kind: String,
    pub identifier: String,
    pub translation: String,
}

impl DictionaryEntry {
    pub fn new(kind: String) -> Self {
        Self {
            kind,
            identifier: String::new(),
            translation: String::new(),
        }
    }
}

impl Display for DictionaryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.identifier, self.translation)
    }
}

#[derive(Clone)]
pub struct Dictionary {
    pub entries: Vec<DictionaryEntry>,
}

impl Dictionary {
    pub fn from_file(mut file: File) -> Self {
        file.seek(SeekFrom::Start(0)).unwrap();
        let file = BufReader::new(file);
        let mut dict = Vec::new();

        let mut parser = Reader::from_reader(file);
        let mut entry = None;
        let mut buf = Vec::new();
        let mut current_tag = String::new();
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
                    current_tag = tag;
                }
                Ok(Event::Text(e)) => {
                    if let Some(entry) = entry.as_mut() {
                        if current_tag == "k" {
                            entry.identifier = e.unescape().unwrap().to_string();
                        }
                        if current_tag == "dtrn" {
                            entry.translation = e.unescape().unwrap().to_string();
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
                    }
                    current_tag = String::new();
                }
                _ => {}
            }
            buf.clear();
        }
        return Dictionary { entries: dict };
    }
}
