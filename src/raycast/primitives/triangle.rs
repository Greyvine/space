use bevy::math::Vec3A;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub v0: Vec3A,
    pub v1: Vec3A,
    pub v2: Vec3A,
}

impl From<(Vec3A, Vec3A, Vec3A)> for Triangle {
    fn from(vertices: (Vec3A, Vec3A, Vec3A)) -> Self {
        Triangle {
            v0: vertices.0,
            v1: vertices.1,
            v2: vertices.2,
        }
    }
}

impl From<Vec<Vec3A>> for Triangle {
    fn from(vertices: Vec<Vec3A>) -> Self {
        Triangle {
            v0: *vertices.get(0).unwrap(),
            v1: *vertices.get(1).unwrap(),
            v2: *vertices.get(2).unwrap(),
        }
    }
}

impl From<[Vec3A; 3]> for Triangle {
    fn from(vertices: [Vec3A; 3]) -> Self {
        Triangle {
            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
        }
    }
}
