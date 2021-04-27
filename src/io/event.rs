use bevy::prelude::*;

pub struct MouseEvent(pub bool);

pub fn listen_mouse(mouse: Res<Input<MouseButton>>, mut event_writer: EventWriter<MouseEvent>) {
    if mouse.just_pressed(MouseButton::Left) {
        event_writer.send(MouseEvent(true));
    } else if mouse.just_released(MouseButton::Left) {
        event_writer.send(MouseEvent(false));
    }
}
