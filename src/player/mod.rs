mod components;
mod systems;

use bevy::prelude::Plugin;

use self::systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player);
    }
}
