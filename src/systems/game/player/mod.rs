use bevy::prelude::*;

use crate::components::bundles::player::PlayerBundle;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server
                .load("images/tilesets/oryx_tiny_galaxy/tg_sliced/tg_monsters/tg_monsters_astronaut_r1.png"),
            sprite: Sprite {
                color: Color::rgb(0., 0., 2.),
                custom_size: Some(Vec2::splat(160.)),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
