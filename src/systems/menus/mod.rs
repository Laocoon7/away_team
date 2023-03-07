use bevy::prelude::App;

pub mod main_menu;
pub mod settings;
pub mod splash_screen;

pub fn add_systems(app: &mut App) {
    splash_screen::add_systems(app);
    main_menu::add_systems(app);
    settings::add_systems(app);
}
