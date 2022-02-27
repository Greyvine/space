#[derive(Default, Debug)]
pub struct RayHit {
    pub distance: f32,
    pub uv_coords: (f32, f32),
}

impl RayHit {
    /// Get a reference to the intersection's uv coords.
    pub fn uv_coords(&self) -> &(f32, f32) {
        &self.uv_coords
    }

    /// Get a reference to the intersection's distance.
    pub fn distance(&self) -> &f32 {
        &self.distance
    }
}
