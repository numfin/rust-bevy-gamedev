pub mod event;

use bevy::prelude::*;

pub struct IOPlugin;
impl Plugin for IOPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<event::MouseEvent>();

        app.add_system(event::listen_mouse.system());
    }
}
