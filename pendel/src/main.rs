use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Shape2dPlugin::default(), MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .add_systems(Update, draw_lines)
        .run();
}


#[derive(Component)]
struct LineBetween {
    start: Entity,
    end: Entity,
}

#[derive(Component)]
struct Anchor;

#[derive(Component)]
struct Object {
    mass: f32,
    size: f32
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    let material = materials.add(ColorMaterial::from(Color::WHITE));

    let anchor = Anchor {};
    let anchor_position = Transform::from_xyz(0., 150., 0.);
    let anchor_mesh = meshes.add(Circle::new(5.));

    let obj = Object {
        mass: 10.,
        size: 20.,
    };
    let obj_position = Transform::from_xyz(0., -50., 0.);
    let obj_mesh = meshes.add(Circle::new(15.));


    let anchor_entity = commands.spawn((
        anchor,
        anchor_position,
        Mesh2d(anchor_mesh),
        MeshMaterial2d(material.clone())
    )).id();

    let object_entity = commands.spawn((
        obj,
        obj_position,
        Mesh2d(obj_mesh),
        MeshMaterial2d(material.clone())
    )).observe(drag_obj_observer).id();

    commands.spawn(LineBetween {
        start: anchor_entity,
        end: object_entity
    });
}


fn update(
    mut obj_pos_query: Query<&mut Transform, With<Object>>
) {
    for mut obj_pos in obj_pos_query.iter_mut() {
        obj_pos.translation.x += 0.1;
    }
}


fn draw_lines(
    line_query: Query<&LineBetween>,
    position_query: Query<&Transform>,
    mut painter: ShapePainter,
) {
    for line in line_query.iter() {
        let Ok(a) = position_query.get(line.start) else { continue };
        let Ok(b) = position_query.get(line.end) else { continue };

        let pos_a = a.translation;
        let pos_b = b.translation;

        painter.set_color(Color::WHITE);
        painter.line(pos_a, pos_b);
    }
}


fn drag_obj_observer(
    drag: Trigger<Pointer<Drag>>,
    mut transform_query: Query<&mut Transform>
) {
    let Ok(mut transform) = transform_query.get_mut(drag.entity()) else { return };

    transform.translation.x += drag.delta.x;
    transform.translation.y -= drag.delta.y;
}