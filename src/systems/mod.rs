mod states;

mod conditions;
mod init;

mod transitions;

use bevy::prelude::*;

use self::states::AppState;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding SystemsPlugin systems");
        app.add_state::<AppState>();
        self.add_continuous_systems(app);
        self.add_splash_screen_plugins(app);
        self.add_main_menu_plugins(app);
        self.add_settings_menu_plugins(app);
        self.add_game_running_plugins(app);
    }
}

impl SystemsPlugin {
    fn add_continuous_systems(&self, _app: &mut App) {}

    fn add_splash_screen_plugins(&self, app: &mut App) {
        app.add_systems((init::assets, init::splash_screen).in_schedule(OnEnter(AppState::SplashScreen)));

        app.add_system(
            transitions::splash_screen_to_main_menu
                .in_set(OnUpdate(AppState::SplashScreen))
                .run_if(conditions::assets_loaded),
        );
    }

    fn add_main_menu_plugins(&self, app: &mut App) {
        app.add_system(test_system.in_schedule(OnEnter(AppState::MainMenu)));
    }

    fn add_settings_menu_plugins(&self, _app: &mut App) {}

    fn add_game_running_plugins(&self, _app: &mut App) {}
}

fn test_system(state: Res<State<AppState>>) {
    info!("We are in the {:?} state", state.0);
}
