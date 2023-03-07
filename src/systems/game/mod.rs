use bevy::prelude::*;

use crate::components::tags::CleanupOnTransitionTag;

use self::player::spawn_player;

use super::states::AppState;

mod player;

pub fn add_systems(app: &mut App) {
    app.add_systems((init_game, spawn_player).in_schedule(OnEnter(AppState::GameRunning)));
    #[cfg(feature = "debug")]
    app.add_system(super::test_system.in_schedule(OnEnter(AppState::GameRunning)));
}

// ################################
// #########       Initialization
// ################################
fn init_game() {
    trace!("Initializing game");
    // TODO: Initialize input
    // TODO: Initialize map
    // TODO: Spawn player
}

// ################################
// #########       Transitions
// ################################
fn to_main_menu(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning game to main menu");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    next_state.set(AppState::MainMenu);
}

// ################################
// #########       Conditions
// ################################
