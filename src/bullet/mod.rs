mod asset;
mod event;
mod state;

use bevy::prelude::*;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(asset::load_bullet_sprite.system());

        app.add_system(state::animate_bullets.system());
        app.add_system(event::spawn_bullets_on_shoot.system());
    }
}
