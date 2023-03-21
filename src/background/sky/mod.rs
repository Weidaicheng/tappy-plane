mod components;
mod systems;

use bevy::prelude::{IntoSystemConfig, Plugin};

use self::systems::{move_sky_left_over_time, spawn_sky, spawn_sky_over_moving_left};

use super::sets::{MovementSystemSet, SpawnSystemSet};

const SKY_WIDTH: f32 = 800.0;
const SKY_HEIGHT: f32 = 480.0;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_sky)
            .add_system(move_sky_left_over_time.in_set(MovementSystemSet))
            .add_system(spawn_sky_over_moving_left.in_set(SpawnSystemSet));
    }
}
