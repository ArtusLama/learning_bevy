use bevy::prelude::*;
use bevy::winit::cursor::CursorIcon;

pub struct CursorHelper {}

impl CursorHelper {
    /// Sets the cursor icon for the primary window.
    pub fn set_cursor_icon(
        mut commands: Commands,
        window: Single<Entity, With<Window>>,
        icon: CursorIcon
    ) {
        commands.
            entity(*window)
            .insert(icon);
    }
}