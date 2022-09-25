use std::fmt::Display;

pub enum AppEvent {
    LoadingStarted,
    DictionaryLoaded,
    FontsLoaded,
    LoadingFinished,
}

impl Display for AppEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::LoadingStarted => "Loading Started",
            Self::LoadingFinished => "Loading Finished",
            Self::DictionaryLoaded => "Dictionary Loaded",
            Self::FontsLoaded => "Fonts loaded",
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