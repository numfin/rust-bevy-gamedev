mod bullet;
mod camera;
mod gun;
mod io;

use bevy::{core::FixedTimestep, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "Kill numfin".into(),
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(io::IOPlugin)
        .add_plugin(gun::GunPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(camera::CameraPlugin)
        .run()
}
