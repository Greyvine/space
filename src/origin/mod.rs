use bevy::prelude::*;

use crate::tag::{NonPlayerTag, PlayerTag};

const MAX_BOUND: f32 = 10_000.0;

#[derive(Component, Default, Clone, Copy)]
pub struct SimulationCoordinates {
    local_translation: Vec3,
    solar_coordinates: Vec3,
    world_coordinates: Vec3,
}

impl SimulationCoordinates {
    pub fn from(pos: Vec3) -> Self {
        Self {
            local_translation: Vec3::new(pos.x % MAX_BOUND, pos.y % MAX_BOUND, pos.z % MAX_BOUND),
            solar_coordinates: Vec3::new(0.0, 0.0, (pos.z / (2.0 * MAX_BOUND)).ceil()),
            world_coordinates: pos,
        }
    }
}

#[derive(Bundle, Default)]
pub struct SimulationBundle {
    transform: Transform,
    simulation_coordinates: SimulationCoordinates,
}

impl SimulationBundle {
    pub fn new(pos: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(pos),
            simulation_coordinates: SimulationCoordinates::from(pos),
        }
    }
}

#[derive(Default)]
pub struct OriginRebasingPlugin;

impl Plugin for OriginRebasingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sync_simulation_coordinates);
    }
}

fn sync_simulation_coordinates(
    mut q: ParamSet<(
        Query<(&Transform, &mut SimulationCoordinates), With<PlayerTag>>,
        Query<(&mut Transform, &SimulationCoordinates), With<SimulationCoordinates>>,
        Query<&mut SimulationCoordinates, With<NonPlayerTag>>,
    )>,
) {
    let mut shift = Vec3::ZERO;

    for (transform, mut simulation_transform) in q.p0().iter_mut() {
        // println!("{}", transform.translation);
        simulation_transform.local_translation = transform.translation;
        if transform.translation.x < -MAX_BOUND {
            simulation_transform.local_translation.x += 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.x -= 1.0;
            shift = 2.0 * Vec3::X * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        } else if transform.translation.x > MAX_BOUND {
            simulation_transform.local_translation.x -= 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.x += 1.0;
            shift = -2.0 * Vec3::X * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        }
        if transform.translation.y < -MAX_BOUND {
            simulation_transform.local_translation.y += 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.y -= 1.0;
            shift = 2.0 * Vec3::Y * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        } else if transform.translation.y > MAX_BOUND {
            simulation_transform.local_translation.y -= 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.y += 1.0;
            shift = -2.0 * Vec3::Y * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        }
        if transform.translation.z < -MAX_BOUND {
            simulation_transform.local_translation.z += 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z -= 1.0;
            shift = 2.0 * Vec3::Z * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        } else if transform.translation.z > MAX_BOUND {
            simulation_transform.local_translation.z -= 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z += 1.0;
            shift = -2.0 * Vec3::Z * MAX_BOUND;
            // rebase_events.send(OrginRebasingEvent::new(&shift));
        }
    }

    if shift != Vec3::ZERO {
        for (mut taransform, npc_simulation_coordinates) in q.p1().iter_mut() {
            // let distance = npc_simulation_coordinates.solar_coordinates.distance(a) * MAX_BOUND;
            // if distance < MAX_VIEW {
            taransform.translation += shift;
            // }
            // else {
            //     let scaling_factor = get_scaling_factor(distance).min(1.0);
            //     let relative_render_position = transform.translation - npc_simulation_coordinates.get_relative_render_position(b);
            //     transform.translation += relative_render_position * scaling_factor;
            //     transform.scale = Vec3::splat(scaling_factor);
            // }
        }
    }
}
