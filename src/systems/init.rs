use bevy::prelude::*;

use crate::components::tags::CleanupOnTransitionTag;

pub fn assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Load images
    // TODO: Load fonts
}

pub fn splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            CleanupOnTransitionTag,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    ..Default::default()
                },
                image: UiImage {
                    texture: asset_server.load("images/splash.png"),
                    flip_x: false,
                    flip_y: false,
                },
                ..Default::default()
            });
        });
}
