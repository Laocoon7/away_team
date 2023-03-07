pub mod settings;

use bevy::prelude::*;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        trace!("Adding ResourcePlugin systems");
        Self::register_types(app);
    }
}

impl ResourcesPlugin {
    fn register_types(_app: &mut App) {
        trace!("Registering Resource types");
        //app.register_type::<>();
    }
}
