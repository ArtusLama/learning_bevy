use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;
use crate::controller::Controller;
use crate::cursor::CursorHelper;

#[derive(Component)]
pub struct DraggableComponent {
    pause_on_interact: bool
}

impl Default for DraggableComponent {
    fn default() -> Self {
        Self {
            pause_on_interact: true
        }
    }
}

impl DraggableComponent {
    pub fn register_for(entity_commands: &mut EntityCommands) {
        entity_commands.observe(click_down_observer);
        entity_commands.observe(click_up_observer);
        entity_commands.observe(start_dragging_observer);
        entity_commands.observe(stop_dragging_observer);
        entity_commands.observe(dragging_observer);
    }
}

// TODO: show grab cursor also on hover!

fn set_grab_cursor(commands: Commands, window: Single<Entity, With<Window>>,) {
    CursorHelper::set_cursor_icon(
        commands,
        window,
        CursorIcon::from(SystemCursorIcon::Pointer)
    );
}

fn set_default_cursor(commands: Commands, window: Single<Entity, With<Window>>,) {
    CursorHelper::set_cursor_icon(
        commands,
        window,
        CursorIcon::from(SystemCursorIcon::Default)
    );
}


fn click_down_observer(
    click: Trigger<Pointer<Down>>,
    mut controller: ResMut<Controller>,
    commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if click.button == PointerButton::Primary {
        controller.is_paused = true;
        set_grab_cursor(commands, window);
    }
}

fn click_up_observer(
    click: Trigger<Pointer<Up>>,
    mut controller: ResMut<Controller>,
    commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if click.button == PointerButton::Primary {
        controller.is_paused = false;
        set_default_cursor(commands, window);
    }
}

fn dragging_observer(
    drag: Trigger<Pointer<Drag>>,
    mut transform_query: Query<&mut Transform>
) {
    if drag.button == PointerButton::Primary {
        let Ok(mut transform) = transform_query.get_mut(drag.entity()) else { return };

        transform.translation.x += drag.delta.x;
        transform.translation.y -= drag.delta.y;
    }
}

fn start_dragging_observer(
    drag_start: Trigger<Pointer<DragStart>>,
    mut controller: ResMut<Controller>,
    commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if drag_start.button == PointerButton::Primary {
        controller.is_paused = true;
        set_grab_cursor(commands, window);
    }
}

fn stop_dragging_observer(
    drag_stop: Trigger<Pointer<DragEnd>>,
    mut controller: ResMut<Controller>,
    commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    if drag_stop.button == PointerButton::Primary {
        controller.is_paused = false;
        set_default_cursor(commands, window);
    }
}