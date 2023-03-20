use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Query, Res, Transform, Vec3, With},
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use crate::background::BACKGROUND_MOVE_SPEED;

use super::{components::Sky, SKY_HEIGHT, SKY_WIDTH};

pub fn spawn_sky(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut spawned_sky_height = 0.0;
    while spawned_sky_height < window.height() {
        let mut spawned_sky_width = 0.0;
        let y = spawned_sky_height + SKY_HEIGHT / 2.0;
        while spawned_sky_width < window.width() {
            let x = spawned_sky_width + SKY_WIDTH / 2.0;
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/sky.png"),
                    ..default()
                },
                Sky {},
            ));
            spawned_sky_width += SKY_WIDTH;
        }
        spawned_sky_height += SKY_HEIGHT;
    }
}

pub fn move_sky_left_over_time(
    mut commands: Commands,
    mut sky_query: Query<(Entity, &mut Transform), With<Sky>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in sky_query.iter_mut() {
        let direction = Vec3::new(-1.0, 0.0, 0.0);
        transform.translation += direction * BACKGROUND_MOVE_SPEED * time.delta_seconds();

        if transform.translation.x + SKY_WIDTH / 2.0 < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_sky_over_moving_left(
    mut commands: Commands,
    sky_query: Query<&Transform, With<Sky>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut rightmost_transform: Option<&Transform> = None;

    for transform in sky_query.iter() {
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

        if transform.translation.x + SKY_WIDTH / 2.0 <= window.width() {
            let mut spawned_sky_height = 0.0;
            let x = window.width() + SKY_WIDTH / 2.0;
            while spawned_sky_height < window.height() {
                let y = spawned_sky_height + SKY_HEIGHT / 2.0;
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0),
                        texture: asset_server.load("sprites/sky.png"),
                        ..default()
                    },
                    Sky {},
                ));
                spawned_sky_height += SKY_HEIGHT;
            }
        }
    }
}
