use super::init::create_gun;
use bevy::prelude::*;

const GUN_SPRITE_SIZE: f32 = 30.0;
pub const GUN_SPRITE_COLUMNS: usize = 4;

pub struct ShotAudioAsset(pub Handle<AudioSource>);
pub struct ReloadAudioAsset(pub Handle<AudioSource>);

pub fn load_gun_sprite(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    let gun_atlas = TextureAtlas::from_grid(
        assets_server.load("gun_sprite.png").into(),
        Vec2::splat(GUN_SPRITE_SIZE),
        GUN_SPRITE_COLUMNS,
        1,
    );

    commands.insert_resource(ShotAudioAsset(assets_server.load("shot.mp3")));
    commands.insert_resource(ReloadAudioAsset(assets_server.load("reload.mp3")));

    create_gun(commands, atlas_assets.add(gun_atlas));
}
