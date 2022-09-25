use bevy::prelude::*;

use crate::app::components::animate::AnimateTranslation;

pub fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32 - 400.0;
        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}