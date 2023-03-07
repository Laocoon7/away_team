use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
pub struct CleanupOnTransitionTag;

#[derive(Component, Reflect, Default, Debug)]
pub struct GameCameraTag;

#[derive(Component, Reflect, Default, Debug)]
pub struct UiCameraTag;

#[derive(Component, Reflect, Default, Debug)]
pub struct PlayerTag;

pub(super) fn register_types(app: &mut App) {
    trace!("Registering Tag types");
    app.register_type::<CleanupOnTransitionTag>();
    app.register_type::<GameCameraTag>();
    app.register_type::<UiCameraTag>();
    app.register_type::<PlayerTag>();
}
