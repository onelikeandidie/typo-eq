use std::fs::File;

use self::base::DictionaryParser;

use super::dictionary::Dictionary;

pub mod base;
pub mod xdxf;
pub mod typo;

pub fn parse_file(file_path: &str) -> Dictionary {
    // Extract extension to determine which parser to use
    let (_name, extension) = file_path.split_at(
        file_path.rfind(".")
        .expect("Could not extract file extension to decide on dictionary parser")
    );
    let file = File::open(file_path)
        .expect("Could not load the dictionary file");
    match extension {
        ".xdxf" => {
            let parser = xdxf::XDXFParser;
            parser.parse(file)
        }
        ".typo" => {
            let parser = typo::TypoEQParser;
            parser.parse(file)
        }
        _ => {
            panic!("Parser does not exist for {} files", extension)
        },
    }
}