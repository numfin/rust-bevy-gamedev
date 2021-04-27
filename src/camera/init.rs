use bevy::prelude::Commands;

use bevy::prelude::*;

pub fn create_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
