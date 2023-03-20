use bevy::prelude::Component;

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
pub struct Player {
    pub propeller_size: PropellerSize,
    pub propeller_size_transform: PropellerSizeTransform,
}
