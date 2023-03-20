mod components;
mod resources;
mod systems;

use bevy::prelude::Plugin;

use self::{
    resources::PropellerRotationTimer,
    systems::{rotate_propeller_over_time, spawn_player, tick_propeller_rotation_timer},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PropellerRotationTimer>()
            .add_startup_system(spawn_player)
            .add_system(tick_propeller_rotation_timer)
            .add_system(rotate_propeller_over_time);
    }
}
