use std::env;

use super::util::get_index;

#[derive(Debug)]
pub struct ConfigFile {
    pub dictionary_path: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub dictionary_path: String,
    pub debugging: bool,
}

pub fn extract_config(args: &Vec<String>) -> Result<Config, String> {
    let mut config = Config {
        dictionary_path: "".to_string(),
        debugging: false,
    };
    // Check if instead, the arguments were passed
    let has_dict = args.contains(&"--dict".to_string());
    if has_dict {
        let index_of_dict = (get_index(&args, "--dict") + 1) as usize;
        let dictionary_path = args
            .get(index_of_dict)
            .unwrap_or(&"./src".to_string())
            .to_owned();
        config.dictionary_path = dictionary_path;
    } else {
        let current_dir = env::current_dir();
        match current_dir {
            Ok(path) => {
                config.dictionary_path = format!("{}{}", path.display(), "dict.xdxf");
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    let debugging = args.contains(&"--debug".to_string());
    config.debugging = debugging;

    return Ok(config);
}
