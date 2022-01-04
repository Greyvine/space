mod event;
pub mod tag;

use self::event::*;
use self::tag::*;
use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TranslationEvent>()
            .add_system(handle_keyboard_input.system())
            .add_system(handle_translation_events.system());
    }
}

fn handle_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut translation_events: EventWriter<TranslationEvent>,
) {
    // let (forward, right, up) = (
    //     (look.forward * xz).normalize(),
    //     (look.right * xz).normalize(),
    //     Vec3::Y,
    // );

    let forward = -Vec3::Z;
    let right = Vec3::X;
    let up = Vec3::Y;

    let mut desired_velocity = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        desired_velocity += forward;
    }
    if keys.pressed(KeyCode::S) {
        desired_velocity -= forward;
    }
    if keys.pressed(KeyCode::D) {
        desired_velocity += right;
    }
    if keys.pressed(KeyCode::A) {
        desired_velocity -= right;
    }
    if keys.pressed(KeyCode::Q) {
        desired_velocity += up;
    }
    if keys.pressed(KeyCode::E) {
        desired_velocity -= up;
    }

    let speed = if keys.pressed(KeyCode::LShift) {
        200.0
    } else {
        0.5
    };

    desired_velocity *= speed;

    // println!("d {}", desired_velocity);
    translation_events.send(TranslationEvent::new(&desired_velocity))
}

fn handle_translation_events(
    mut events: EventReader<TranslationEvent>,
    mut query: Query<&mut Transform, With<ControllerPlayerTag>>,
) {
    for event in events.iter().next() {
        for mut transform in query.iter_mut() {
            transform.translation += **event;
            // println!("e {}", **event);
        }
    }
}
