use glam::{Vec3, Quat};

#[derive(Clone, Copy)]
pub struct Transform
{
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform
{
    pub const IDENTITY: Self = Self
    {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };
}