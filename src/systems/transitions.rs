use bevy::prelude::*;

use crate::components::tags::CleanupOnTransitionTag;

pub fn splash_screen_to_main_menu(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
) {
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
