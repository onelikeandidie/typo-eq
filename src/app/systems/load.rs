use std::fs::File;
use bevy::prelude::*;

use crate::{app::{events::AppEvent, resources::{AppState, AppAssets}}, importer::dictionary, config::Config};


pub fn load_dictionary(
    config: Res<Config>, 
    mut app_state: ResMut<AppState>,
    mut app_event: EventWriter<AppEvent>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<AppAssets>,
) {
    app_event.send(AppEvent::LoadingStarted);
    let font: Handle<Font> = asset_server.load("fonts/JetBrains Mono Medium Nerd Font Complete Mono Windows Compatible.ttf");
    let font_id = font.id;
    assets.fonts.insert("Jetbrains".to_string(), font_id);
    app_event.send(AppEvent::FontsLoaded);
    let file = File::open(config.dictionary_path.clone())
        .expect("Could not load the dictionary file");
    let dict = dictionary::Dictionary::from_file(file);
    app_state.dict = Some(dict);
    app_event.send(AppEvent::DictionaryLoaded);
    app_event.send(AppEvent::LoadingFinished);
}