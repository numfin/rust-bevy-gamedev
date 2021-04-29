use bevy::{prelude::*, render::camera::OrthographicProjection};

pub struct Bullet;

fn easing(x: f32) -> f32 {
    1.0 - f32::powf(1.0 - x, 3.0)
}

pub fn animate_bullets(
    time: Res<Time>,
    query_bullet: Query<(&mut Transform, &mut Timer), With<Bullet>>,
) {
    query_bullet.for_each_mut(|(mut transform, mut timer)| {
        timer.tick(time.delta());
        let p = timer.percent();
        transform.translation.x = easing(p) * 1000.0;
    })
}

pub fn destroy_bullets(
    query_bullet: Query<(Entity, &Transform, &Handle<ColorMaterial>), With<Bullet>>,
    mut assets: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    camera: Query<&OrthographicProjection>,
) {
    let border_right = match camera.single() {
        Ok(c) => c.right,
        Err(_) => 0.0,
    };

    query_bullet.for_each(|(entity, transform, asset)| {
        if transform.translation.x > border_right {
            commands.entity(entity).despawn_recursive();
            assets.remove(asset);
        }
    })
}
