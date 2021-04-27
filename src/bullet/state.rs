use bevy::prelude::*;

pub struct Bullet;

pub fn animate_bullets(
    time: Res<Time>,
    query: Query<(&mut Transform, Entity, &Handle<ColorMaterial>), With<Bullet>>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    query.for_each_mut(|(mut transform, entity, material)| {
        transform.translation.x += time.delta_seconds() * 3000.0;
        if transform.translation.x >= 500.0 {
            assets.remove(material);
            commands.entity(entity).despawn_recursive();
        }
    })
}
