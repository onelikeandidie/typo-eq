use bevy::prelude::Component;

#[derive(Component)]
pub struct Word {
    pub progress: usize,
    pub size: usize,
    pub original: String,
    pub translation: String,
}

#[derive(Component)]
pub struct FocusedWord;

#[derive(Component)]
pub struct TranslatedWord;