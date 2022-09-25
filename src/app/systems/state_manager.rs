use bevy::prelude::*;

use crate::{app::{resources::{AppState, AppAssets}, components::{word::{Word, FocusedWord, TranslatedWord}, animate::AnimateTranslation}, events::ProgressEvent}};
use rand::{prelude::*, distributions::Uniform};

pub fn word_generator(
    mut commands: Commands,
    app_state: Res<AppState>,
    asset_server: Res<AssetServer>,
    assets: Res<AppAssets>,
    query: Query<&Word>,
) {
    if query.iter().count() > 0 {
        return;
    }
    let font_id = assets.fonts.get(&"Jetbrains".to_string()).unwrap();
    let font: Handle<Font> = asset_server.get_handle(*font_id);
    let font_size = 42.0;
    let text_style = TextStyle {
        color: Color::WHITE,
        font: font.clone(),
        font_size,
    };
    let fail_text_style = TextStyle {
        color: Color::ORANGE_RED,
        font: font.clone(),
        font_size,
    };
    let completed_text_style = TextStyle {
        color: Color::SEA_GREEN,
        font: font.clone(),
        font_size,
    };
    let translated_text_style = TextStyle {
        color: Color::WHITE,
        font: font.clone(),
        font_size: 28.0,
    };
    if let Some(dict) = &app_state.dict {
        let mut rng = thread_rng();
        let distribuition = Uniform::new(0, dict.entries.len());
        let word_index = distribuition.sample(&mut rng);
        let word = dict.entries.get(word_index);
        if let Some(word) = word {
            println!("{}", word);
            let translated_word_id = commands.spawn()
                .insert(TranslatedWord)
                .insert_bundle(Text2dBundle {
                    text: Text::from_section(
                        word.translation.clone(), 
                        translated_text_style
                    ).with_alignment(TextAlignment::CENTER),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: -42.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                }).id();
            commands.spawn()
                .insert(Word {
                    original: word.identifier.clone(),
                    translation: word.translation.clone(),
                    progress: 0,
                    size: word.identifier.len(),
                }).insert(FocusedWord)
                .insert_bundle(Text2dBundle {
                    text: Text::from_sections([
                        TextSection::from_style(completed_text_style),
                        TextSection::from_style(fail_text_style),
                        TextSection {
                            value: word.identifier.clone(),
                            style: text_style.clone(),
                            ..Default::default()
                        },
                    ]).with_alignment(TextAlignment::CENTER),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                }).add_child(translated_word_id);
        }
    }
}

pub fn word_progress_manager(
    mut commands: Commands,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut query: Query<(Entity, &mut Word), With<FocusedWord>>,
    mut progress_events: EventWriter<ProgressEvent>
) {
    if char_evr.is_empty() {
        return;
    }
    let text_input = char_evr.iter();
    let text_input = text_input.map(|c| c.char).collect::<Vec<char>>();
    for (entity, mut word) in query.iter_mut() {
        let current_word: &String = &word.original;
        let chars: Vec<char> = current_word.chars().collect::<Vec<char>>();
        if word.progress >= (word.size - 1) {
            commands.entity(entity).despawn_recursive();
        }
        for input in text_input.clone() {
            let current_char: Option<&char> = chars
                .get(word.progress);
            if let Some(current_char) = current_char {
                if current_char == &input {
                    word.progress += 1;
                    progress_events.send(ProgressEvent::Success(Some(input)));
                } else {
                    progress_events.send(ProgressEvent::Fail);
                }
            }
        }
    }
}

pub fn word_progress_display_updater(
    mut query: Query<&mut Text, With<FocusedWord>>,
    mut progress_events: EventReader<ProgressEvent>,
) {
    if progress_events.is_empty() || query.is_empty() {
        return;
    }
    for event in progress_events.iter() {
        match event {
            ProgressEvent::Success(_) => {
                for mut text in query.iter_mut() {
                    let sections: &Vec<TextSection> = &mut text.sections;
                    let src_section = if sections[1].value.len() != 0 {
                        1
                    } else {
                        2
                    };
                    let mut src = sections[src_section].clone();
                    let mut chars = src.value.chars();
                    if let Some(c) = chars.next() {
                        let mut s2 = sections[0].clone();
                        s2.value.push(c);
                        text.sections[0] = s2;
                    }
                    src.value = chars.as_str().to_string();
                    text.sections[src_section] = src;
                    println!("{}:{}:{}", text.sections[0].value, text.sections[1].value, text.sections[2].value);
                }
            },
            ProgressEvent::Fail => {
                for mut text in query.iter_mut() {
                    let sections: &Vec<TextSection> = &mut text.sections;
                    if sections[1].value.len() != 0 {
                        continue;
                    }
                    let mut s1 = sections[2].clone();
                    let mut chars = s1.value.chars();
                    if let Some(c) = chars.next() {
                        let mut s2 = sections[1].clone();
                        s2.value.push(c);
                        text.sections[1] = s2;
                    }
                    s1.value = chars.as_str().to_string();
                    text.sections[2] = s1;
                    println!("{}:{}:{}", text.sections[0].value, text.sections[1].value, text.sections[2].value);
                }
            }
        }
    }
}