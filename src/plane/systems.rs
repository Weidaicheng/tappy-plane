use std::time::Duration;

use bevy::{
    prelude::{
        default, AssetServer, Commands, Handle, Image, Input, KeyCode, Quat, Query, Res, ResMut,
        Transform, Vec3, With,
    },
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use super::{
    components::{Plane, PropellerSize, PropellerSizeTransform},
    resources::{PlaneDropTimer, PropellerRotationTimer, PLANE_DROP_TIME},
    PLANE_CLIMB_SPEED, PLANE_DROP_SPEED, PLANE_HEIGHT,
};

fn get_asset_path(propeller_size: PropellerSize) -> String {
    match propeller_size {
        PropellerSize::Large => "sprites/planeYellow1.png".to_string(),
        PropellerSize::Middle => "sprites/planeYellow2.png".to_string(),
        PropellerSize::Tiny => "sprites/planeYellow3.png".to_string(),
    }
}

pub fn spawn_plane(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let x = window.width() / 5.0;
    let y = window.height() / 2.0;

    let plane = Plane {
        direction: Vec3::new(0.0, 0.0, 0.0),
        propeller_size: PropellerSize::Large,
        propeller_size_transform: PropellerSizeTransform::Decrease,
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load(get_asset_path(plane.propeller_size)),
            ..default()
        },
        plane,
    ));
}

pub fn tick_timer(
    mut propeller_rotation_timer: ResMut<PropellerRotationTimer>,
    mut plane_drop_timer: ResMut<PlaneDropTimer>,
    time: Res<Time>,
) {
    propeller_rotation_timer.timer.tick(time.delta());
    plane_drop_timer.timer.tick(time.delta());
}

pub fn rotate_propeller_over_time(
    mut plane_query: Query<(&mut Handle<Image>, &mut Plane)>,
    propeller_rotation_timer: Res<PropellerRotationTimer>,
    asset_server: Res<AssetServer>,
) {
    if propeller_rotation_timer.timer.finished() {
        for (mut handle, mut plane) in plane_query.iter_mut() {
            plane.propeller_size = match plane.propeller_size {
                PropellerSize::Middle => match plane.propeller_size_transform {
                    PropellerSizeTransform::Increase => PropellerSize::Large,
                    PropellerSizeTransform::Decrease => PropellerSize::Tiny,
                },
                _ => PropellerSize::Middle,
            };
            plane.propeller_size_transform = match plane.propeller_size_transform {
                PropellerSizeTransform::Increase => PropellerSizeTransform::Decrease,
                PropellerSizeTransform::Decrease => PropellerSizeTransform::Increase,
            };
            *handle = asset_server.load(get_asset_path(plane.propeller_size));
        }
    }
}

pub fn drop_plane_period(
    mut plane_query: Query<(&mut Transform, &mut Plane)>,
    plane_drop_timer: Res<PlaneDropTimer>,
) {
    if plane_drop_timer.timer.finished() {
        for (mut transform, mut plane) in plane_query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(270.0);
            plane.direction = Vec3::new(0.0, -1.0, 0.0);
        }
    }
}

pub fn climb_plane(
    mut plane_query: Query<(&mut Transform, &mut Plane)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (mut transform, mut plane) in plane_query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(120.0);
            plane.direction = Vec3::new(0.0, 1.0, 0.0);
        }
    }
}

pub fn plane_movement_over_time(
    mut plane_query: Query<(&mut Transform, &Plane)>,
    mut plane_drop_timer: ResMut<PlaneDropTimer>,
    time: Res<Time>,
) {
    for (mut transform, plane) in plane_query.iter_mut() {
        if plane.direction.y < 0.0 {
            // dropping
            plane_drop_timer.timer.reset();
            transform.translation += plane.direction * PLANE_DROP_SPEED * time.delta_seconds();
        } else if plane.direction.y > 0.0 {
            // climbing
            plane_drop_timer
                .timer
                .tick(Duration::from_secs_f32(PLANE_DROP_TIME / 50.0));
            transform.translation += plane.direction * PLANE_CLIMB_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_plane_movement(
    mut plane_query: Query<&mut Transform, With<Plane>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for mut transform in plane_query.iter_mut() {
        if transform.translation.y - PLANE_HEIGHT / 2.0 < 0.0 {
            transform.translation.y = PLANE_HEIGHT / 2.0;
        }
        if transform.translation.y + PLANE_HEIGHT / 2.0 > window.height() {
            transform.translation.y = window.height() - PLANE_HEIGHT / 2.0;
        }
    }
}
