mod controller;
mod dragging;
mod settings;
mod cursor;

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::controller::Controller;
use crate::dragging::DraggableComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Fadenpendel | Physics Simulation".into(),
                    name: Some("Fadenpendel | Physics Simulation".into()),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins((Shape2dPlugin::default(), MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    commands.insert_resource(Controller::default());


    let material = materials.add(ColorMaterial::from(Color::WHITE));
    let mesh = meshes.add(Circle::new(15.));

    let mut obj = commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        DraggableComponent::default()
    ));

    DraggableComponent::register_for(&mut obj)


}


