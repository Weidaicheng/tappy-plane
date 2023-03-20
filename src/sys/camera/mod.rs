mod systems;

use bevy::prelude::Plugin;

use self::systems::spawn_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_camera);
    }
}
