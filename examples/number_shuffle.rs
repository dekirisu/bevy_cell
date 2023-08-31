use bevy_cell::*;
use bevy::prelude::*;

handle_cell!{@Vec #load #random Image: numbers;}
entity_cell!{slot}

fn startup (
    mut cmd: Commands,
    assets: Res<AssetServer>
){
    cmd.spawn(Camera2dBundle::default());
    // loads num_000.png - num_009.png and attach it as Vec<Handle<Image>>
    // into the Image type. Image::get_numbers(index) returns the Handle now.
    Image::set_numbers((&assets,"num",0..=9,"png").into());
    // Saves the spawned Entity into the Entity type. Entity::get_slot()
    // returns this entity now anywhere.
    Entity::set_slot(cmd.spawn(SpriteBundle{
        texture: Image::get_numbers_random(),
        ..default()
    }).id());
}

fn update (
    mut timer: ResMut<ChangeTimer>,
    time: Res<Time>,
    mut qu: Query<&mut Handle<Image>>
){
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // Change the Handle<Image> of the Slot-Entities to a random number-image.
        *qu.get_mut(Entity::get_slot()).unwrap() = Image::get_numbers_random(); 
    }
}

#[derive(Resource)]
struct ChangeTimer(Timer);

fn main () {
    let mut app = App::new();
    app
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,startup)
    .add_systems(Update,update)
    .insert_resource(ChangeTimer(Timer::from_seconds(1.,TimerMode::Repeating)))
    .run();
}