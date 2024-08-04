// systems/camera_movement.rs
use crate::components::camera_controller::CameraController;
use bevy::prelude::*;

pub fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&CameraController, &mut Transform)>,
) {
    for (controller, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        let translation = &mut transform.translation;
        *translation += time.delta_seconds() * direction * controller.speed;
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), CameraController::default()));
}
