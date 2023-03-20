mod background;
mod player;
mod sys;

use background::BackgroundPlugin;
use bevy::{prelude::App, DefaultPlugins};
use player::PlayerPlugin;
use sys::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
