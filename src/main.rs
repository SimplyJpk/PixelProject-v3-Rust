use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use components::util::fps_text::{setup as setup_fps_text, text_update_system};
mod constant;
use systems::physics::sand_step::sand_step;
mod components;
mod systems;

const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;

const FIXED_UPDATE: f64 = 1.0 / 64.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
                    .with_scale_factor_override(1.0),
                title: "Pixel Project v3".to_string(),
                ..default()
            }),
            ..default()
        }),
    )
    // Add FPS and diagnostics plugins
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    // Resources
    .insert_resource(Time::<Fixed>::from_seconds(FIXED_UPDATE))
    // Startup
    .add_systems(Startup, systems::world::setup_world)
    .add_systems(Startup, systems::camera_movement::setup_camera)
    .add_systems(Startup, setup_fps_text)
    // Fixed Update
    .add_systems(FixedUpdate, sand_step)
    // Update
    .add_systems(Update, text_update_system)
    .add_systems(Update, systems::camera_movement::camera_movement_system)
    // Exit
    .add_systems(Update, systems::exit_on_esc::exit_on_esc_system)
    // Run
    .run();
}
