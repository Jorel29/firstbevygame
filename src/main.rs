use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
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

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer:ResMut<GreetTimer>, query: Query<&Name, With<Person>>){
    //update timer with time elapsed since last update
    //if the update caused timer to finish, say hello
    if timer.0.tick(time.delta()).just_finished(){
        for name in &query{
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>){
    for mut name in &mut query {
        if name.0 == "Elaina Proctor"{
            name.0 = "Elaina Hume".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update,  (update_people, greet_people).chain()); 
    }
}