use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::components::tags::{GameCameraTag, UiCameraTag};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding CameraPlugin systems");
        app.add_startup_systems((
            init_game_camera,
            //init_ui_camera // TODO: RenderTargets
        ));
    }
}

fn init_game_camera(mut commands: Commands) {
    info!("Spawning Game Camera");
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // HDR is required for bloom
                ..Default::default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        },
        BloomSettings::default(),
        GameCameraTag,
    ));
}

fn init_ui_camera(mut commands: Commands) {
    info!("Spawning UI Camera");
    commands.spawn((Camera2dBundle::default(), UiCameraTag));
}
