mod components;
mod resources;
mod systems;

use bevy::prelude::Plugin;

use self::{
    resources::{PlaneDropTimer, PropellerRotationTimer},
    systems::{
        climb_plane, confine_plane_movement, drop_plane_period, plane_movement_over_time,
        rotate_propeller_over_time, spawn_plane, tick_plane_drop_timer,
        tick_propeller_rotation_timer,
    },
};

const PLANE_HEIGHT: f32 = 73.0;
const PLANE_DROP_SPEED: f32 = 50.0;
const PLANE_CLIMB_SPEED: f32 = 100.0;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PropellerRotationTimer>()
            .init_resource::<PlaneDropTimer>()
            .add_startup_system(spawn_plane)
            .add_system(tick_propeller_rotation_timer)
            .add_system(rotate_propeller_over_time)
            .add_system(tick_plane_drop_timer)
            .add_system(drop_plane_period)
            .add_system(plane_movement_over_time)
            .add_system(climb_plane)
            .add_system(confine_plane_movement);
    }
}
