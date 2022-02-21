use super::{ray::Ray3d, RayCastMethod, RayCastSource};
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn compute_ray<T: 'static + Send + Sync>(
    mut source_query: Query<(&mut RayCastSource<T>, Option<&GlobalTransform>)>,
) {
    for (mut source, transform) in &mut source_query.iter_mut() {
        source.ray = match &mut source.cast_method {
            RayCastMethod::Transform => {
                let transform_matrix = transform
                    .expect("The Transform has no associated GlobalTransform")
                    .compute_matrix();
                Some(Ray3d::from(transform_matrix))
            }
            RayCastMethod::Screenspace(_) => todo!(),
        }
    }
}
