use bevy_cell::*;
use bevy::{prelude::*, utils::HashMap};

entities!{
    single, [vec], {hashmap:u32},
    (single_e@Image), [vec_e@Mesh], {hashmap_e@Scene:u32}
}
fn entities (){
    let e = Entity::from_raw(0);
    Entity::set_single(e);
    Entity::set_vec([e]);
    Entity::set_hashmap([(200,e)]);
    Image::set_single_e(e);
    Mesh::set_vec_e([e]);
    Scene::set_hashmap_e([(200,e)]);
    assert_eq!(Entity::single(),Image::single_e());
    assert_eq!(Entity::vec(0),Scene::hashmap_e(&200));
}

handles!{
    Mesh: single, [vec], {hashmap:u32}, (single@AudioSource);
    StandardMaterial: single, [vec], {hashmap:u32}, {hashmap_mats@Mesh:u32};
}
fn handles (app: &mut App){
    let mut res = app.world.resource_mut::<Assets<Mesh>>();
    let m = res.add(shape::Cube::new(1.).into());
    Mesh::set_single(m.clone());
    Mesh::set_vec([m.clone()]);
    Mesh::set_hashmap([(200,m.clone())]);
    AudioSource::set_single(m.clone());
    assert_eq!(Mesh::single(),AudioSource::single());
    
    let mut res = app.world.resource_mut::<Assets<StandardMaterial>>();
    let m = res.add(Color::RED.into());
    StandardMaterial::set_single(m.clone());
    StandardMaterial::set_vec([m.clone()]);
    StandardMaterial::set_hashmap([(200,m.clone())]);
    Mesh::set_hashmap_mats([(200,m.clone())]);
    assert_eq!(StandardMaterial::vec(0),Mesh::hashmap_mats(&200));
}

fn main(){
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    entities();
    handles(&mut app);
}