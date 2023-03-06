mod debug;
mod log;

use bevy::{
    app::{NoopPluginGroup, PluginGroupBuilder},
    prelude::*,
    window::{WindowMode, WindowResolution},
};

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK));
    app.add_plugins(default_plugins());
    app.add_plugins(external_plugins());
    app.add_plugins(my_plugins());
    register_types(&mut app);
    app
}

fn default_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                position: WindowPosition::At(IVec2 { x: 0, y: 0 }),
                resolution: WindowResolution::new(1920., 1080.),
                title: "Away Team".to_string(),
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.,
                    min_height: 600.,
                    ..Default::default()
                },
                resizable: true,
                decorations: true,
                focused: true,
                canvas: None,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
        .set(log::my_log_plugin())
}

fn external_plugins() -> PluginGroupBuilder {
    NoopPluginGroup.build()
}

fn my_plugins() -> PluginGroupBuilder {
    let plugins = PluginGroupBuilder::start::<bevy::app::NoopPluginGroup>();
    #[cfg(feature = "debug")]
    let plugins = plugins.add(debug::DebugPlugin);

    plugins
}

fn register_types(_app: &mut App) {
    //app.register_type::<>();
}
