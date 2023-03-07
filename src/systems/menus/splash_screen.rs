use bevy::prelude::*;

use crate::{components::tags::CleanupOnTransitionTag, systems::states::AppState};

pub fn add_systems(app: &mut App) {
    app.add_systems((init_splash_screen, init_assets).in_schedule(OnEnter(AppState::SplashScreen)));

    app.add_system(to_main_menu.in_set(OnUpdate(AppState::SplashScreen)).run_if(check_assets_loaded));

    #[cfg(feature = "debug")]
    app.add_system(super::super::test_system.in_schedule(OnEnter(AppState::SplashScreen)));
}

// ################################
// #########       Initialization
// ################################
fn init_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("Initializing splash screen");
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

fn init_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("Initializing assets");
    // TODO: Load images
    // TODO: Load fonts
}

// ################################
// #########       Transitions
// ################################
fn to_main_menu(
    mut commands: Commands,
    query_cleanup: Query<Entity, With<CleanupOnTransitionTag>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    trace!("Transitioning splash screen to main menu");
    for entity in query_cleanup.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // TODO: switch to AppState::MainMenu when main menu is available
    next_state.set(AppState::GameRunning);
    // next_state.set(AppState::MainMenu);
}

// ################################
// #########       Conditions
// ################################
fn check_assets_loaded(mut commands: Commands) -> bool {
    trace!("Waiting for assets to be loaded");
    // Wait for assets to load
    true
}
