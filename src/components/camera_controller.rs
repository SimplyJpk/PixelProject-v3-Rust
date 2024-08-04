use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self { speed: 500.0 }
    }
}
