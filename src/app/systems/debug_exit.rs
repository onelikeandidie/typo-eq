use bevy::{prelude::{Res, KeyCode, EventWriter}, input::Input, app::AppExit};

pub fn debug_exit(
    input: Res<Input<KeyCode>>, 
    mut app_exit_events: EventWriter<AppExit>
) {
    if input.pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}