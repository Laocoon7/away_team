use bevy::prelude::*;

use crate::{components::tags::CleanupOnTransitionTag, systems::states::AppState};

pub fn add_systems(app: &mut App) {
    app.add_systems((init_main_menu,).in_schedule(OnEnter(AppState::MainMenu)));
    #[cfg(feature = "debug")]
    app.add_system(super::super::test_system.in_schedule(OnEnter(AppState::MainMenu)));
}

// ################################
// #########       Initialization
// ################################
fn init_main_menu() {
    trace!("Initializing main menu");
}

// ################################
// #########       Transitions
// ################################
fn to_settings(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning main menu to settings");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(AppState::SettingsMenu);
}

fn to_game(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning main menu to game");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(AppState::GameRunning);
}

// ################################
// #########       Conditions
// ################################
