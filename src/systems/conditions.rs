use bevy::prelude::*;

pub fn assets_loaded(mut commands: Commands) -> bool {
    trace!("Waiting for assets to be loaded");
    // Wait for assets to load
    true
}
