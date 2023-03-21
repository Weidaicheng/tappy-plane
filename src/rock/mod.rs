mod components;
mod systems;

use bevy::prelude::Plugin;

use self::systems::spawn_rock;

const ROCK_WIDTH: f32 = 108.0;
const ROCK_HEIGHT: f32 = 239.0;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_rock);
    }
}
