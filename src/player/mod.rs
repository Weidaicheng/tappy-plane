mod components;
mod resources;
mod systems;

use bevy::prelude::Plugin;

use self::{
    resources::{PlaneDropTimer, PropellerRotationTimer},
    systems::{
        drop_plane_when_time_count_down, rotate_propeller_over_time, spawn_player,
        tick_plane_drop_timer, tick_propeller_rotation_timer, move_plane_over_time,
    },
};

const PLAYER_MOVE_SPEED: f32 = 50.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PropellerRotationTimer>()
            .init_resource::<PlaneDropTimer>()
            .add_startup_system(spawn_player)
            .add_system(tick_propeller_rotation_timer)
            .add_system(rotate_propeller_over_time)
            .add_system(tick_plane_drop_timer)
            .add_system(drop_plane_when_time_count_down)
            .add_system(move_plane_over_time);
    }
}
