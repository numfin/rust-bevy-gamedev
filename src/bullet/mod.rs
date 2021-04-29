mod asset;
mod event;
mod state;

use bevy::prelude::*;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // app.add_stage_after(
        //     CoreStage::Update,
        //     "stage1",
        //     SystemStage::parallel()
        //         .with_run_criteria(FixedTimestep::step(0.5))
        //         .with_system(state::animate_bullets.system()),
        // );
        app.add_startup_system(asset::load_bullet_sprite.system());

        app.add_system(state::animate_bullets.system());
        app.add_system(state::destroy_bullets.system());
        app.add_system(event::spawn_bullets_on_shoot.system());
    }
}
