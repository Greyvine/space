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

#[derive(Component)]
pub struct LodLowTag;

#[derive(Component)]
pub struct LodMedTag;

#[derive(Component)]
pub struct LodHighTag;

pub fn lod_system(
    mut commands: Commands,
    mut q: QuerySet<(
        QueryState<(&Transform, &SimulationCoordinates), With<PlayerTag>>,
        QueryState<
            (
                Entity,
                &Children,
                &Transform,
            ),
            With<LodTag>,
        >,
    )>,
    mut lod_q: QuerySet<(
        QueryState<&mut Visibility, With<LodLowTag>>,
        QueryState<&mut Visibility, With<LodMedTag>>,
        QueryState<&mut Visibility, With<LodHighTag>>,
    )>,
) {
    let mut player_transform: Vec3 = Vec3::ZERO;

    for (transform, _) in &mut q.q0().iter() {
        player_transform = transform.translation;
    }

    for (entity, children, transform) in q.q1().iter() {
        // let mut mesh = meshes.get_mut(mesh_handle).unwrap();

        // let new_mesh_handle: Handle<Mesh> = asset_server.get_handle(lod_bundle.low.clone());
        // let new_mesh = meshes.get_mut(new_mesh_handle).unwrap();

        // let ee = commands.entity(entity).get_handle::<LodLowTag>();
        let distance = player_transform.distance(transform.translation);

        if distance < 10.0 {
            println!("CLOSE");

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q0().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q1().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q2().get_mut(*child) {
                    visibility.is_visible = true;
                }
            }
        } else if distance < 50.0 {
            println!("MEDIUM");

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q0().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q1().get_mut(*child) {
                    visibility.is_visible = true;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q2().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }
        } else {
            println!("FAR");

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q0().get_mut(*child) {
                    visibility.is_visible = true;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q1().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }

            for child in children.iter() {
                if let Ok(mut visibility) = lod_q.q2().get_mut(*child) {
                    visibility.is_visible = false;
                }
            }
        }

    }
}
