use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Query, Res, Transform, Vec3, With},
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use crate::background::BACKGROUND_MOVE_SPEED;

use super::{components::Ground, GROUND_HEIGHT, GROUND_WIDTH};

pub fn spawn_ground(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut spawned_ground_width = 0.0;
    while spawned_ground_width < window.width() {
        let x = spawned_ground_width + GROUND_WIDTH / 2.0;
        let y = GROUND_HEIGHT / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/groundDirt.png"),
                ..default()
            },
            Ground {},
        ));

        spawned_ground_width += GROUND_WIDTH;
    }
}

pub fn move_ground_left_over_time(
    mut commands: Commands,
    mut ground_query: Query<(Entity, &mut Transform), With<Ground>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in ground_query.iter_mut() {
        let direction = Vec3::new(-1.0, 0.0, 0.0);
        transform.translation += direction * BACKGROUND_MOVE_SPEED * time.delta_seconds();

        if transform.translation.x + GROUND_WIDTH / 2.0 < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_ground_over_moving_left(
    mut commands: Commands,
    ground_query: Query<&Transform, With<Ground>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut rightmost_transform: Option<&Transform> = None;

    for transform in ground_query.iter() {
        if let Some(temp_transform) = rightmost_transform {
            if transform.translation.x > temp_transform.translation.x {
                rightmost_transform = Some(transform);
            }
        } else {
            rightmost_transform = Some(transform);
        }
    }

    if let Some(transform) = rightmost_transform {
        let window = window_query.get_single().unwrap();

        if transform.translation.x + GROUND_WIDTH / 2.0 <= window.width() {
            let x = window.width() + GROUND_WIDTH / 2.0;
            let y = GROUND_HEIGHT / 2.0;
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/groundDirt.png"),
                    ..default()
                },
                Ground {},
            ));
        }
    }
}
