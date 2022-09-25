use std::collections::HashMap;

use bevy::asset::HandleId;

use crate::importer::dictionary::Dictionary;

#[derive(Clone, Default)]
pub struct AppAssets {
    pub fonts: HashMap<String, HandleId>
}

#[derive(Clone, Default)]
pub struct AppState {
    pub dict: Option<Dictionary>,
    pub count: u32,
}