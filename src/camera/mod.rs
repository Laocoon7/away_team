use bevy::prelude::*;

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
    commands.spawn((Camera2dBundle::default(), GameCameraTag));
}

fn init_ui_camera(mut commands: Commands) {
    info!("Spawning UI Camera");
    commands.spawn((Camera2dBundle::default(), UiCameraTag));
}
