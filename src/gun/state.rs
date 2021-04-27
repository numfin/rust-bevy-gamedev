use bevy::prelude::*;

use super::{
    asset::GUN_SPRITE_COLUMNS,
    event::{ReloadEvent, ShootEvent},
};

#[derive(Eq, PartialEq)]
pub enum GunState {
    Idle,
    Shooting,
    Ending,
}

pub struct Gun(pub GunState);

pub fn animate_gun(
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut Gun)>,
    mut shoot_event_writer: EventWriter<ShootEvent>,
    mut reload_event_writer: EventWriter<ReloadEvent>,
    time: Res<Time>,
) {
    if let Ok((mut timer, mut sprite, mut gun)) = query.single_mut() {
        if timer.tick(time.delta()).finished() {
            let next = (sprite.index + 1) % GUN_SPRITE_COLUMNS as u32;

            if gun.0 == GunState::Shooting {
                sprite.index = next;
                if next == 1 {
                    shoot_event_writer.send(ShootEvent);
                }
            }
            if gun.0 == GunState::Ending {
                match next {
                    1 => {
                        gun.0 = GunState::Idle;
                        reload_event_writer.send(ReloadEvent);
                    }
                    _ => sprite.index = next,
                }
            }
        }
    }
}
