mod camera;

use bevy::{DefaultPlugins, prelude::App};
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .run();
}
