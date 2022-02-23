use bevy::math::Vec3A;

use super::Triangle;

pub trait TriangleTrait {
    fn v0(&self) -> Vec3A;
    fn v1(&self) -> Vec3A;
    fn v2(&self) -> Vec3A;
    fn to_triangle(self) -> Triangle;
}
impl TriangleTrait for [Vec3A; 3] {
    fn v0(&self) -> Vec3A {
        self[0]
    }
    fn v1(&self) -> Vec3A {
        self[1]
    }
    fn v2(&self) -> Vec3A {
        self[2]
    }

    fn to_triangle(self) -> Triangle {
        Triangle::from(self)
    }
}
impl TriangleTrait for Triangle {
    fn v0(&self) -> Vec3A {
        self.v0
    }

    fn v1(&self) -> Vec3A {
        self.v1
    }

    fn v2(&self) -> Vec3A {
        self.v2
    }

    fn to_triangle(self) -> Triangle {
        self
    }
}
