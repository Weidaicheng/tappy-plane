use bevy::{
    prelude::{default, AssetServer, Commands, Query, Res, Transform, With},
    sprite::SpriteBundle,
    window::{PrimaryWindow, Window},
};

use super::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x = window.width() / 5.0;
    let y = window.height() / 2.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("sprites/planeYellow1.png"),
            ..default()
        },
        Player {},
    ));
}
