use bevy::prelude::*;
use std::marker::PhantomData;

use super::{primitive::Intersection, ray::Ray3d, RayCastMethod};

#[derive(Component)]
pub struct RayCastSource<T> {
    pub cast_method: RayCastMethod,
    pub(crate) ray: Option<Ray3d>,
    intersections: Vec<(Entity, Intersection)>,
    _marker: PhantomData<T>,
}

impl<T> Default for RayCastSource<T> {
    fn default() -> Self {
        RayCastSource {
            cast_method: RayCastMethod::Screenspace(Vec2::ZERO),
            ray: None,
            intersections: Vec::new(),
            _marker: PhantomData::default(),
        }
    }
}

impl<T> RayCastSource<T> {
    /// Instantiates a [RayCastSource] with [RayCastMethod::Transform], and an empty ray. It will not
    /// be initialized until the [update_raycast] system is run and a [GlobalTransform] is present on
    /// this entity.
    /// # Warning
    /// Only use this if the entity this is associated with will have its [Transform] or
    /// [GlobalTransform] specified elsewhere. If the [GlobalTransform] is not set, this ray casting
    /// source will never be able to generate a raycast.
    pub fn new_transform_empty() -> Self {
        RayCastSource {
            cast_method: RayCastMethod::Transform,
            ..Default::default()
        }
    }
}
