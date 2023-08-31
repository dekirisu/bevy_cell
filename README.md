<p align="center">
    <a href="https://github.com/dekirisu/bevy_cell" style="position:relative">
        <img src="https://img.shields.io/badge/github-dekirisu/bevy_cell-ee6677">
    </a>
    <a href="https://crates.io/crates/bevy_cell" style="position:relative">
        <img src="https://img.shields.io/crates/v/bevy_cell">
    </a>
    <a href="https://docs.rs/bevy_cell" style="position:relative">
        <img src="https://img.shields.io/docsrs/bevy_cell">
    </a>
    <a href="https://discord.gg/kevWvBuPFg" style="position:relative">
        <img src="https://img.shields.io/discord/515100001903312898">
    </a>
</p>

Easily attach <a href="https://github.com/bevyengine/bevy">bevy</a>'s Handles/Entities statically to types on startup and get them in any system, without using Resources.

It's just a little less clutter, and a bit more descriptive: e.g. `Image::get_icon(Icon::Play)` would return a weak clone of the Handle set with `Image::set_icon(img_handle)`. The setter can only be used once! This is only useful if you're sure these Handles/Entities will never change (and are always available)! For a more general use case, see <a href="https://github.com/dekirisu/type_cell">type_cell</a>.

## Overview
```rust 
use bevy_cell::*;
// setup cells
handle_cell!{Image: handle_a;}
entity_cell!{entity_a}
// on startup
Image::set_handle_a(img_handle);
Entity::set_entity_a(entity);
// in any system
Image::get_handle_a();
Entity::get_entity_a();
```
The macro also accepts multiple definitions at once:
```rust 
handle_cell!{
    Image: image_a, image_b, image_c;
    Mesh: mesh_a, mesh_b;
}
handle_cell!{ @Vec
    Image: image_vec_a, image_vec_b, image_vec_c;
    Mesh: mesh_vec_a, mesh_vec_b;
}
entity_cell!{
    entity_a, entity_b, entity_c
}
```
For Vec/HashMaps:
```rust 
handle_cell!{@Vec Image: numbers;}
handle_cell!{@HashMap<K> Image: numbers_id;}
entity_cell!{@Vec slots}
entity_cell!{@HashMap<K> slots_id}
// on startup
Image::set_numbers(img_handle_vec);
Image::set_numbers_id(img_handle_map);
Entity::set_slots(entity_vec);
Entity::set_slots_id(entity_map);
// in any system
Image::get_numbers(index);
Image::get_numbers_id(key);
Entity::get_slots(index);
Entity::get_slots_id(key);
```
## Simple Example
```rust
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

```
## Example, using some experimental features
In handle_cell:<br>
@Vec: save a Vec of handles, instead of just one<br>
#load: the setter will load multiple assets from the folder, which are numbered.<br>
#random: include a getter, which returns a random Handle of this Vec.

```rust
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

```