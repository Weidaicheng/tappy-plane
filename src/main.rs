mod background;
mod plane;
mod rock;
mod sys;

use background::BackgroundPlugin;
use bevy::{prelude::App, DefaultPlugins};
use plane::PlanePlugin;
use rock::RockPlugin;
use sys::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlanePlugin)
        .add_plugin(RockPlugin)
        .run();
}
