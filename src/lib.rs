mod camera;
mod components;
mod debug;
mod log;
mod resources;
mod systems;

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
    let plugins = PluginGroupBuilder::start::<NoopPluginGroup>();

    plugins
}

fn my_plugins() -> PluginGroupBuilder {
    let plugins = PluginGroupBuilder::start::<NoopPluginGroup>()
        .add(camera::CameraPlugin)
        .add(components::ComponentsPlugin)
        .add(systems::SystemsPlugin);

    #[cfg(feature = "debug")]
    let plugins = plugins.add(debug::DebugPlugin);

    plugins
}

fn register_types(_app: &mut App) {
    // See components::register_types()
    // See resources::register_types()

    //app.register_type::<>();
}
