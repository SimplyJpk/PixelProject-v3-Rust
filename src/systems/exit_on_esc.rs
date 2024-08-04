use bevy::prelude::*;

pub fn exit_on_esc_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
