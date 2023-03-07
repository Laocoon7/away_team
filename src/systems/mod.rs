pub mod states;

mod game;
mod menus;

use bevy::prelude::*;

use self::states::AppState;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding SystemsPlugin systems");
        app.add_state::<AppState>();
        self.add_continuous_systems(app);
        menus::add_systems(app);
        game::add_systems(app);
    }
}

impl SystemsPlugin {
    fn add_continuous_systems(&self, _app: &mut App) {}
}

#[cfg(feature = "debug")]
pub fn test_system(state: Res<State<AppState>>) {
    info!("We are in the {:?} state", state.0);
}
