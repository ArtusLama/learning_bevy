use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((Shape2dPlugin::default(), MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update)
        .add_systems(Update, draw_lines)
        .add_systems(FixedUpdate, (apply_accelerations, apply_velocity))
        .run();
}

#[derive(Resource)]
struct GameController {
    pause_physics: bool,
    transform_multiplier: f32
}

impl Default for GameController {
    fn default() -> Self {
        Self {
            pause_physics: false,
            transform_multiplier: 0.001
        }
    }
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
    size: f32,
    velocity: Vec2
}


#[derive(Component)]
struct Acceleration {
    value: Vec2
}

impl Acceleration {
    pub fn get_gravity() -> Self {
        Self {
            value: Vec2::new(0., -9.81)
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

    commands.insert_resource(GameController::default());

    let material = materials.add(ColorMaterial::from(Color::WHITE));

    let anchor = Anchor {};
    let anchor_position = Transform::from_xyz(0., 150., 0.);
    let anchor_mesh = meshes.add(Circle::new(5.));

    let obj = Object {
        mass: 10.,
        size: 20.,
        velocity: Vec2::ZERO
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
        MeshMaterial2d(material.clone()),
        Acceleration::get_gravity()
    ))
        .observe(obj_click_down_observer)
        .observe(obj_click_up_observer)
        .observe(drag_obj_observer)
        .observe(start_dragging_obj_observer)
        .observe(stop_dragging_obj_observer)
        .id();

    commands.spawn(LineBetween {
        start: anchor_entity,
        end: object_entity
    });
}

fn apply_accelerations(
    mut query: Query<(&Acceleration, &mut Object)>,
    game_controller: Res<GameController>
) {
    if game_controller.pause_physics { return }

    for (force, mut object) in query.iter_mut() {
        object.velocity += force.value;
    }
}

fn apply_velocity(
    mut query: Query<(&Object, &mut Transform)>,
    game_controller: Res<GameController>
) {
    if game_controller.pause_physics { return }

    for (object, mut transform) in query.iter_mut() {
        transform.translation += object.velocity.extend(0.) * game_controller.transform_multiplier;
    }
}

fn update(
    mut obj_query: Query<&mut Object>,
    game_controller: Res<GameController>
) {
    for obj in obj_query.iter_mut() {

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


fn obj_click_down_observer(
    click: Trigger<Pointer<Down>>,
    mut game_controller: ResMut<GameController>
) {
    if click.button == PointerButton::Primary {
        game_controller.pause_physics = true
    }
}

fn obj_click_up_observer(
    click: Trigger<Pointer<Up>>,
    mut game_controller: ResMut<GameController>
) {
    if click.button == PointerButton::Primary {
        game_controller.pause_physics = false
    }
}

fn drag_obj_observer(
    drag: Trigger<Pointer<Drag>>,
    mut transform_query: Query<&mut Transform, With<Object>>
) {
    if drag.button == PointerButton::Primary {
        let Ok(mut transform) = transform_query.get_mut(drag.entity()) else { return };

        transform.translation.x += drag.delta.x;
        transform.translation.y -= drag.delta.y;
    }
}

fn start_dragging_obj_observer(
    drag_start: Trigger<Pointer<DragStart>>,
    mut game_controller: ResMut<GameController>
) {
    if drag_start.button == PointerButton::Primary {
        game_controller.pause_physics = true
    }
}

fn stop_dragging_obj_observer(
    drag_stop: Trigger<Pointer<DragEnd>>,
    mut game_controller: ResMut<GameController>
) {
    if drag_stop.button == PointerButton::Primary {
        game_controller.pause_physics = false
    }
}