use bevy::prelude::*;

pub struct BulletAsset(pub HandleUntyped);

pub fn load_bullet_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BulletAsset(asset_server.load_untyped("bullet.png")));
}
