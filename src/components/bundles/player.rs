use bevy::prelude::*;

use crate::components::tags::PlayerTag;

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteBundle,
    pub player_tag: PlayerTag,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle::default(),
            player_tag: PlayerTag,
        }
    }
}
