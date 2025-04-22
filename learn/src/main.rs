use bevy::prelude::*;

pub struct MyPlugin {}

impl Plugin for MyPlugin  {
    fn build(&self, app: &mut App) {
        todo!()
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        /*.add_systems(Update, (
            greet_world,
            (update_people, greet_people).chain()
        ))*/
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Max Mustermann".to_string())));
    commands.spawn((Person, Name("Test Person".to_string())));
}

fn greet_world() {
    println!("Hello World!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0)
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Test Person" {
            name.0 = "Perfect Person".to_string();
            break;
        }
    }
}






fn main1() {
    App::new()
        //   Type of System    What to run
        .add_systems(Startup, greet_system)
        .run();
}

fn greet_system() {
    println!("Hello World!")
}
