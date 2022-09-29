use std::env;
use typo_eq::{
    config::{extract_config, Config},
    app,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config;
    match extract_config(&args) {
        Ok(imported_config) => config = imported_config,
        Err(error) => panic!("{}", error),
    }

    app::create_app(config);
}
