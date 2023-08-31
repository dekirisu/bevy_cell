use bevy_cell::*;
use bevy::prelude::*;

handle_cell!{Image: number;}
entity_cell!{slot}

fn startup (
    mut cmd: Commands,
    assets: Res<AssetServer>
){
    cmd.spawn(Camera2dBundle::default());
    // set Handles/Entities once on startup
    Image::set_number(assets.load("num_005.png"));
    Entity::set_slot(cmd.spawn(SpriteBundle{
        texture: assets.load("num_000.png"),
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
        // get Handles/Entities anywhere
        *qu.get_mut(Entity::get_slot()).unwrap() = Image::get_number(); 
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
    .insert_resource(ChangeTimer(Timer::from_seconds(1.,TimerMode::Once)))
    .run();
}