use bevy::prelude::*;

use crate::{origin::SimulationCoordinates, tag::PlayerTag};

#[derive(Component, Default)]
pub struct LodBundle {
    high: String,
    med: String,
    low: String,
}

impl LodBundle {
    pub fn new(high: String, med: String, low: String) -> Self {
        Self {
            high: high,
            med: med,
            low: low,
        }
    }
}

#[derive(Component)]
pub struct LodTag;

pub fn lod_system(
    mut q: QuerySet<(
        QueryState<(&Transform, &SimulationCoordinates), With<PlayerTag>>,
        QueryState<(&Transform, &SimulationCoordinates, &LodBundle), With<LodTag>>,
    )>,
) {
    let mut player_transform: Vec3 = Vec3::ZERO;

    for (transform, _) in &mut q.q0().iter() {
        player_transform = transform.translation;
    }

    for (transform, sim_coord, lod_bundle) in q.q1().iter() {
        let distance = player_transform.distance(transform.translation);

        if distance < 10.0 {
            println!("CLOSE");
        } else if distance < 50.0 {
            println!("MEDIUM");
        } else {
            println!("FAR");
        }
    }
}
