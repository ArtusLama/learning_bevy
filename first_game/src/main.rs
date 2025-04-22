use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod settings;
use settings::*;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set((WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("My first game!"),
                        resolution: WindowResolution::new(800., 600.),
                        ..default()
                    }),
                    ..default()
                }))
        )
        .add_plugins(EguiPlugin)
        .add_systems(Update, e_gui_system)
        .add_systems(Startup, setup)
        .add_systems(Update, on_mouse_click)
        .add_systems(FixedUpdate, move_player)
        .run();
}

fn e_gui_system(
    mut contexts: EguiContexts,
    player_query: Query<&Transform, With<Player>>
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        for pos in player_query.iter() {
            ui.label(pos.translation.to_string());
        }
    });
}




#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);



fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}


fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: Vec2
) {
    let player: Player = Player {};
    let mesh = meshes.add(PLAYER_SHAPE);
    let material = materials.add(ColorMaterial::from_color(PLAYER_COLOR));

    commands.spawn((
        player,
        Transform::from_translation(pos.extend(0.)),
        Velocity(Vec2::ZERO),
        Mesh2d(mesh),
        MeshMaterial2d(material)
    ));
}


fn move_player(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    let attraction = 0.1;

    // Collect all positions and velocities into a temporary vector
    let entities: Vec<_> = query.iter().map(|(transform, velocity)| (transform.translation, velocity.0)).collect();

    for (mut position, mut velocity) in &mut query {
        let current_position = position.translation;

        let vel: Vec2 = entities
            .iter()
            .filter(|(other_position, _)| *other_position != current_position) // Exclude itself
            .map(|(other_position, _)| {
                let direction = *other_position - current_position;
                direction.truncate().normalize_or_zero() * attraction
            })
            .sum();

        velocity.0 += vel;

        position.translation += velocity.0.extend(0.);
    }
}

fn on_mouse_click(
    mut mouse_input: EventReader<MouseButtonInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for event in mouse_input.read() {
        if event.state == ButtonState::Pressed && event.button == MouseButton::Left {
            if let Ok(window) = windows.get_single() {
                if let Some(cursor_position) = window.cursor_position() {
                    if let Ok((camera, camera_transform)) = cameras.get_single() {
                        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                            spawn_player(&mut commands, &mut meshes, &mut materials, world_position);
                        }
                    }
                }
            }
            return;
        }
    }
}