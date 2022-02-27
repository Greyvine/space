use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component, Debug)]
pub struct RayCastMesh<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for RayCastMesh<T> {
    fn default() -> Self {
        RayCastMesh {
            _marker: PhantomData::default(),
        }
    }
}
