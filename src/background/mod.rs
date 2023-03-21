mod ground;
mod sets;
mod sky;

use bevy::prelude::{IntoSystemSetConfigs, Plugin};

use self::{
    ground::GroundPlugin,
    sets::{MovementSystemSet, SpawnSystemSet},
    sky::SkyPlugin,
};

const BACKGROUND_MOVE_SPEED: f32 = 100.0;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets((MovementSystemSet, SpawnSystemSet).chain())
            .add_plugin(GroundPlugin)
            .add_plugin(SkyPlugin);
    }
}
