use std::ops::Mul;
use glam::Vec3A;

struct Ray
{
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray
{
    fn new(origin: Vec3A, direction: Vec3A) -> Self
    {
        Self{ origin, direction }
    }

    fn at(&self, t: f32) -> Vec3A
    {
        return self.origin + self.direction.mul(t);
    }
}