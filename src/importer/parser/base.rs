use std::fs::File;

use crate::importer::dictionary::Dictionary;

pub trait DictionaryParser {
    fn parse(self: &Self, file: File) -> Dictionary;
}
