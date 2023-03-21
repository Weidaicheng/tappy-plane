use bevy::prelude::{Component, Vec3};

#[derive(Clone, Copy)]
pub enum PropellerSize {
    Large,
    Middle,
    Tiny,
}

#[derive(Clone, Copy)]
pub enum PropellerSizeTransform {
    Decrease,
    Increase,
}

#[derive(Component, Clone, Copy)]
pub struct Plane {
    pub direction: Vec3,
    pub propeller_size: PropellerSize,
    pub propeller_size_transform: PropellerSizeTransform,
}
