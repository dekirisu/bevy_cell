use bevy_cell::*;
use bevy::prelude::*;

entity_cell!{@Vec #random slots}
handle_cell!{@Vec #load #random Image: numbers;}

#[derive(Resource)]
struct ChangeTimer(Timer);

fn main () {
    let mut app = App::new();
    app
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,startup)
    .add_systems(Update,update)
    .insert_resource(ChangeTimer(Timer::from_seconds(2.,TimerMode::Repeating)))
    .run();
}

fn startup (
    mut cmd: Commands,
    assets: Res<AssetServer>
){
    cmd.spawn(Camera2dBundle{
        transform: Transform::from_xyz(65.*4.5, 65.*4.5, 1000.0-0.1),
        ..default()
    });
    Image::set_numbers(HandleVecLoad::new(&assets,"num",0..=9,"png"));
    let mut vec = Vec::with_capacity(10*10);
    for x in 0..10 { for y in 0..10 {
        vec.push(cmd.spawn(SpriteBundle{
            texture: Image::get_numbers_random(),
            transform: Transform::from_xyz(x as f32 * 65., y as f32 * 65., 1.),
            ..default()
        }).id());
    }}
    Entity::set_slots(vec);
}

fn update (
    mut timer: ResMut<ChangeTimer>,
    time: Res<Time>,
    mut qu: Query<&mut Handle<Image>>
){
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for e in Entity::get_slots_vec() {
            *qu.get_mut(*e).unwrap() = Image::get_numbers_random();
        }
    }
}