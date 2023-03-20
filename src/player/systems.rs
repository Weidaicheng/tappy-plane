use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Query, Res, ResMut, Transform, With},
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use super::{
    components::{Player, PropellerSize, PropellerSizeTransform},
    resources::PropellerRotationTimer,
};

fn get_asset_path(propeller_size: PropellerSize) -> String {
    match propeller_size {
        PropellerSize::Large => "sprites/planeYellow1.png".to_string(),
        PropellerSize::Middle => "sprites/planeYellow2.png".to_string(),
        PropellerSize::Tiny => "sprites/planeYellow3.png".to_string(),
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x = window.width() / 5.0;
    let y = window.height() / 2.0;

    let player = Player {
        propeller_size: PropellerSize::Large,
        propeller_size_transform: PropellerSizeTransform::Decrease,
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load(get_asset_path(player.propeller_size)),
            ..default()
        },
        player,
    ));
}

pub fn tick_propeller_rotation_timer(
    mut propeller_rotation_timer: ResMut<PropellerRotationTimer>,
    time: Res<Time>,
) {
    propeller_rotation_timer.timer.tick(time.delta());
}

pub fn rotate_propeller_over_time(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &Player)>,
    propeller_rotation_timer: Res<PropellerRotationTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if propeller_rotation_timer.timer.finished() {
        for (entity, _, player) in player_query.iter() {
            let window = window_query.get_single().unwrap();

            let x = window.width() / 5.0;
            let y = window.height() / 2.0;
            commands.entity(entity).insert((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load(get_asset_path(player.propeller_size)),
                    ..default()
                },
                Player {
                    propeller_size: match player.propeller_size {
                        PropellerSize::Middle => match player.propeller_size_transform {
                            PropellerSizeTransform::Increase => PropellerSize::Large,
                            PropellerSizeTransform::Decrease => PropellerSize::Tiny,
                        },
                        _ => PropellerSize::Middle,
                    },
                    propeller_size_transform: match player.propeller_size_transform {
                        PropellerSizeTransform::Increase => PropellerSizeTransform::Decrease,
                        PropellerSizeTransform::Decrease => PropellerSizeTransform::Increase,
                    },
                },
            ));
        }
    }
}
