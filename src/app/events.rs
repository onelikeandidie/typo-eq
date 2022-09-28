use std::fmt::Display;

use crate::importer::dictionary::Dictionary;

pub enum AppEvent {
    LoadingStarted,
    DictionaryLoaded(Dictionary),
    LoadingFinished,
}

impl Display for AppEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::LoadingStarted => "Loading Started",
            Self::LoadingFinished => "Loading Finished",
            Self::DictionaryLoaded(_) => "Dictionary Loaded",
        })
    }
}

pub enum ProgressEvent {
    Fail,
    Success(Option<char>),
}

impl Display for ProgressEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Fail => "Failed",
            Self::Success(_) => "Success",
        })
    }
}