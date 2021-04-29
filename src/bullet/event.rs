use super::{asset::BulletAsset, state::Bullet};
use crate::gun::event::ShootEvent;
use bevy::prelude::*;

pub fn spawn_bullets_on_shoot(
    mut commands: Commands,
    mut event_reader: EventReader<ShootEvent>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    bullet: Res<BulletAsset>,
) {
    event_reader.iter().for_each(|_| {
        commands
            .spawn_bundle(SpriteBundle {
                material: assets.add(bullet.0.clone_weak().typed().into()),
                transform: Transform {
                    translation: Vec3::new(40.0, 25.0, 0.0),
                    scale: Vec3::splat(2.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet)
            .insert(Timer::from_seconds(0.5, false));
    });
}
