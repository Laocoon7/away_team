use std::io::Cursor;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

fn _set_window_icon(windows: NonSend<WinitWindows>, primary_query: Query<Entity, With<PrimaryWindow>>) {
    let window_entity = primary_query.single();
    let primary = windows.get_window(window_entity).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let icon_buf = Cursor::new(include_bytes!("../../../assets/images/icon.png"));
        let rgba =
            image::load(icon_buf, image::ImageFormat::Png).expect("Failed to open icon path").into_rgba8();

        let (width, height) = rgba.dimensions();
        let icon_raw = rgba.into_raw();
        (icon_raw, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

fn main() {
    let mut app = away_team::app();

    info!("Starting launcher: Native");
    //app.add_startup_system(set_window_icon);
    app.run();
}
