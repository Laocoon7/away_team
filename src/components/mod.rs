pub mod tags;

use bevy::prelude::*;

pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding ComponentPlugin systems");
        Self::register_types(app);
    }
}

impl ComponentsPlugin {
    fn register_types(app: &mut App) {
        trace!("Registering Component types");
        tags::register_types(app);
        //app.register_type::<>();
    }
}
