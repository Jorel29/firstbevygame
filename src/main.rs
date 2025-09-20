use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn hello_world(){
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("John Rubicon".to_string())));
    commands.spawn((Person, Name("Maxwell Sting".to_string())));
    commands.spawn((Person, Name("Rapy Lace".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query{
        println!("hello {}!", name.0);
    }
}