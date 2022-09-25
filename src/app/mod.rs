pub mod events;
pub mod resources;
pub mod systems;
pub mod components;

use crate::config::Config;

use bevy::prelude::*;

use self::{events::{AppEvent, ProgressEvent}, resources::{AppState, AppAssets}};

pub fn create_app(config: Config) {
    App::new()
        .add_event::<AppEvent>()
        .add_event::<ProgressEvent>()
        .insert_resource(WindowDescriptor {
            width:  854.0,
            height: 576.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .insert_resource(config)
        .insert_resource(AppAssets::default())
        .insert_resource(AppState::default())
        .add_startup_system(systems::setup::setup)
        .add_startup_system(systems::load::load_dictionary)
        .add_system(systems::debug_exit::debug_exit)
        // .add_system(systems::input::keyboard_input_system)
        // .add_system(systems::debug_systems::animate_translation)
        .add_system(systems::state_manager::word_generator)
        .add_system(systems::state_manager::word_progress_manager)
        .add_system(systems::state_manager::word_progress_display_updater)
        .run();
}