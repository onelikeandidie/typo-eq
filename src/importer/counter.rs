use std::{fs::File, io::Read};

use regex::Regex;

pub fn count_xml_tags(mut file: File) -> usize {
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Cannot read file");
    let regex = Regex::new(r"<[^/\n]+>").unwrap();
    let matches = regex.find_iter(buf.as_str()).count();
    return matches;
}
