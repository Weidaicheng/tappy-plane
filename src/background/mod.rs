mod ground;

use bevy::prelude::{IntoSystemConfig, IntoSystemSetConfigs, Plugin, SystemSet};

use self::ground::systems::{
    move_ground_left_over_time, spawn_ground, spawn_ground_over_moving_left,
};

const BACKGROUND_MOVE_SPEED: f32 = 100.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct SpawnSystemSet;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_ground)
            .configure_sets((MovementSystemSet, SpawnSystemSet).chain())
            .add_system(move_ground_left_over_time.in_set(MovementSystemSet))
            .add_system(spawn_ground_over_moving_left);
    }
}
