use bevy::prelude::*;

use super::state::{Gun, GunState};

pub fn create_gun(mut commands: Commands, gun_atlas: Handle<TextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: gun_atlas,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.025, true))
        .insert(Gun(GunState::Idle));
}
