use bevy::prelude::*;

use crate::{components::tags::CleanupOnTransitionTag, systems::states::AppState};

pub fn add_systems(app: &mut App) {
    app.add_systems((init_settings_menu,).in_schedule(OnEnter(AppState::SettingsMenu)));
    #[cfg(feature = "debug")]
    app.add_system(super::super::test_system.in_schedule(OnEnter(AppState::SettingsMenu)));
}

// ################################
// #########       Initialization
// ################################
fn init_settings_menu() {
    trace!("Initializing settings menu");
}

// ################################
// #########       Transitions
// ################################
fn to_main_menu(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning settings to main menu");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(AppState::MainMenu);
}

// ################################
// #########       Conditions
// ################################
