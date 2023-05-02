use std::ops::Mul;
use glam::Vec3A;
use std::fmt;
use std::fmt::Formatter;

pub struct Ray
{
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray
{
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self
    {
        Self{ origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3A
    {
        return self.origin + self.direction.mul(t);
    }
}

impl fmt::Display for Ray
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "origin: {}\ndirection: {}", self.origin, self.direction);
    }
}