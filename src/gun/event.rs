// use super::asset::ReloadAudioAsset;
use super::{
    asset::ShotAudioAsset,
    state::{Gun, GunState},
};
use crate::io::event::MouseEvent;
use bevy::prelude::*;

/// Emmited when pistol shot
pub struct ShootEvent;
/// Emmited when pistol stop shootin
pub struct ReloadEvent;

pub fn listen_gun_input(mut event_reader: EventReader<MouseEvent>, mut query: Query<&mut Gun>) {
    event_reader.iter().for_each(|event| {
        if let Ok(mut gun) = query.single_mut() {
            gun.0 = match event.0 {
                true => GunState::Shooting,
                false => GunState::Ending,
            }
        }
    })
}

pub fn listen_gun_shot(
    mut event_reader: EventReader<ShootEvent>,
    audio: Res<Audio>,
    audio_asset: Res<ShotAudioAsset>,
) {
    event_reader.iter().for_each(|_| {
        audio.play(audio_asset.0.clone());
    })
}
pub fn listen_gun_reload(
    mut event_reader: EventReader<ReloadEvent>,
    audio: Res<Audio>,
    // audio_asset: Res<ReloadAudioAsset>,
) {
    // event_reader.iter().for_each(|_| {
    //     audio.play(audio_asset.0.clone());
    // })
}
