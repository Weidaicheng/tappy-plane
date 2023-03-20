use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

const PROPELLER_ROTATION_TIME: f32 = 0.1;
const PLANE_DROP_TIME: f32 = 2.0;

#[derive(Resource)]
pub struct PropellerRotationTimer {
    pub timer: Timer,
}

impl Default for PropellerRotationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(PROPELLER_ROTATION_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct PlaneDropTimer {
    pub timer: Timer,
}

impl Default for PlaneDropTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(PLANE_DROP_TIME, TimerMode::Once),
        }
    }
}
