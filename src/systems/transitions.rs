use bevy::prelude::*;

use crate::components::tags::CleanupOnTransitionTag;

use super::states::AppState;

pub fn splash_screen_to_main_menu(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning splash screen to main menu");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(AppState::MainMenu);
}
