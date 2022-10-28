use std::env;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::collections::hash_map::HashMap;
use std::path::Path;

use super::util::get_index;

#[derive(Debug)]
pub struct ConfigFile {
    pub dictionary_path: String,
    pub show_phrases: bool, 
    pub profile: String,
}

#[derive(Debug, Clone)]
pub struct ProfileFile {
    pub path: String,
    pub profiles: HashMap<String, Profile>,
}


impl ProfileFile {
    pub fn new(path: String) -> Self {
        ProfileFile {
            path,
            profiles: HashMap::new()
        }
    }
    pub fn load(path: String) -> Result<Self, String> {
        let file = File::open(path.clone());
        if let Ok(mut file) = file {
            let mut contents = String::new(); 
            if let Err(_) = file.read_to_string(&mut contents) {
                return Err("Could not read profile file to string".to_string());
            }
            let mut lines = contents.lines();
            println!("{}", path);
            let mut profile_file = ProfileFile{ path, profiles: HashMap::new()};
            let mut current_profile: Option<Profile> = None;
            while let Some(line) = lines.next() {
                // Profiles look like this: "[default]"
                if line.starts_with("[") && line.ends_with("]") {
                    let profile_name = line.replace("[", "");
                    let profile_name = profile_name.replace("]", "");
                    let new_profile = Some(Profile {
                        name: profile_name,
                        words_learnt: HashMap::new(),
                    });
                    // If there is a profile currently in the stack, make
                    // sure to add it to the file
                    if let Some(profile) = current_profile {
                        profile_file.profiles.insert(profile.name.clone(), profile);
                    }
                    current_profile = new_profile;
                    continue;
                }
                // Every line after that profile declaration is a word
                // with their complete count divided by a # like so:
                // "some_word#13"
                // The word is  "some_word" and it was completed 13 times
                if let Some(profile) = current_profile.clone() {
                    let mut profile = profile;
                    let mut line_separated = line.split("#");
                    let word = line_separated.next();
                    let count = line_separated.next();
                    if let Some(word) = word {
                        // If the word does not have a count or that count
                        // could not be parsed, default to 0
                        let count = if let Some(count) = count {
                            count.parse::<i64>().unwrap_or(0)
                        } else {
                            0
                        };
                        profile.words_learnt.insert(word.to_string(), count);
                        current_profile = Some(profile);
                    }
                }
            }
            if let Some(profile) = current_profile {
                // Push remaining profile after read
                profile_file.profiles.insert(profile.name.clone(), profile);
            } else {
                // Default if there is no profiles
                profile_file.profiles.insert("default".to_string(), Profile {
                    words_learnt: HashMap::new(),
                    name: "default".to_string(),
                });
            }
            Ok(profile_file)
        } else {
            Err(format!("Profile files could not be opened on: {}", path.clone()))
        }
    }
    pub fn save(self: &Self) ->  Result<(), String> {
        if let Some(dir) = Path::new(&self.path).parent() {
            DirBuilder::new().recursive(true).create(dir).unwrap();
        }
        let file = File::create(self.path.clone());
        match file {
            Ok(file) => {
                self.save_to_file(file)
            },
            Err(err) => {
                Err(format!(
                    "Profile files could not be saved to: {} (Err: {:?})", 
                    self.path.clone(), 
                    err
                ))
            },
        }
    }
    pub fn save_to_file(self: &Self, mut file: File) ->  Result<(), String> {
        for profile in self.profiles.values() {
            file.write_all(format!("[{}]\n", profile.name).as_bytes())
                .expect("Profile could not be saved on profile file");
            for (word, count) in &profile.words_learnt {
                file.write_all(format!("{}#{}\n", word, count).as_bytes())
                    .expect("Could not write word into profile")
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub words_learnt: HashMap<String, i64>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub dictionary_path: String,
    pub show_phrases: bool, 
    pub profile: Profile,
    pub profile_file: ProfileFile,
    pub debugging: bool,
}

pub fn extract_config(args: &Vec<String>) -> Result<Config, String> {
    // Check if the dict file was set or use default
    let has_dict = args.contains(&"--dict".to_string()) || args.contains(&"-d".to_string());
    let dictionary_path;
    if has_dict {
        let index_of_dict = (get_index(&args, "--dict") + 1) as usize;
        dictionary_path = args
            .get(index_of_dict)
            .unwrap_or(&"./src".to_string())
            .to_owned();
    } else {
        let current_dir = env::current_dir();
        match current_dir {
            Ok(path) => {
                dictionary_path = format!("{}{}", path.display(), "dict.xdxf");
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    let show_phrases = args.contains(&"--phrases".to_string()) || args.contains(&"-p".to_string());
    let debugging = args.contains(&"--debug".to_string());

    let has_profile = args.contains(&"--profile".to_string());
    let mut profile = Profile {
        name: "default".to_string(),
        words_learnt: HashMap::new(),
    };
    // Import profile from profile_file
    let home = env::var("HOME").expect("Cannot load profiles if $HOME is not set");
    let path = format!("{}/{}", home, ".config/typo-eq/profiles.txt");
    let profile_file;
    if has_profile {
        let index_of_profile = (get_index(&args, "--profile") + 1) as usize;
        let profile_name = args.get(index_of_profile)
            .unwrap_or(&"default".to_string())
            .to_owned();
        profile.name = profile_name.clone();
        let file = ProfileFile::load(path.clone());
        let saved_profile = if let Ok(file) = file {
            profile_file = file;
            if let Some(p) = profile_file.profiles.get(&profile_name) {
               Some(p.clone()) 
            } else {None}
        } else {
            profile_file = ProfileFile::new(path);
            None
        };
        if let Some(saved_profile) = saved_profile {
            profile = saved_profile;
        }
    } else {
        profile_file = ProfileFile::new(path);
    }

    Ok(Config {
        dictionary_path,
        debugging,
        show_phrases,
        profile,
        profile_file,
    })
}
