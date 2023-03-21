mod components;
mod systems;

use bevy::prelude::{IntoSystemConfig, Plugin};

use self::systems::{move_ground_left_over_time, spawn_ground, spawn_ground_over_moving_left};

use super::sets::{MovementSystemSet, SpawnSystemSet};

const GROUND_WIDTH: f32 = 808.0;
const GROUND_HEIGHT: f32 = 71.0;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_ground)
            .add_system(move_ground_left_over_time.in_set(MovementSystemSet))
            .add_system(spawn_ground_over_moving_left.in_set(SpawnSystemSet));
    }
}
