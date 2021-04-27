mod asset;
pub mod event;
mod init;
mod state;

use bevy::prelude::*;

pub struct GunPlugin;
impl Plugin for GunPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<event::ShootEvent>();
        app.add_event::<event::ReloadEvent>();

        app.add_startup_system(asset::load_gun_sprite.system());

        app.add_system(state::animate_gun.system());
        app.add_system(event::listen_gun_input.system());
        app.add_system(event::listen_gun_shot.system());
        app.add_system(event::listen_gun_reload.system());
    }
}
