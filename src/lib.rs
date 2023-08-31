pub use type_cell::*;
pub use std::ops::RangeInclusive;
use rand::Rng;
use bevy::{prelude::*, asset::Asset};
use intuple::*;
use derive_new::new;

#[macro_export]
macro_rules! entity_cell {
    ($($name: ident),*) => {type_cell!{#clone Entity:$($name),*;}};
    (@Vec $($name: ident),*) => {type_cell!{@Vec #unwrap #clone Entity:$($name),*;}};

    (@Vec #random $($name: ident),*) => {
        $(type_cell!(on Entity > store Vec<Entity> | set Vec<Entity> | 
            get.get(id:usize).unwrap().clone() Entity, 
            vec get &'static Vec<Entity>, 
            random get.get_random().clone() Entity
        |$name);)*
    };

    (@HashMap<$id:ty> $($name: ident),*) => {type_cell!{@HashMap<$id> #unwrap #clone Entity:$($name),*;}};
}

#[macro_export]
macro_rules! handle_cell {
    ($($ty:ty: $($name: ident),*;)*) => {type_cell!{#clone $($ty>Handle<$ty>:$($name),*;)*}};
    (@Vec $($ty:ty: $($name: ident),*;)*) => {type_cell!{@Vec #unwrap #clone $($ty>Handle<$ty>:$($name),*;)*}};

    (@Vec #random $($ty:ty: $($name: ident),*;)*) => {
        $($(type_cell!(on $ty > store Vec<Handle<$ty>> | set Vec<Handle<$ty>> | 
            get.get(id:usize).unwrap().clone() Handle<$ty>, 
            vec get &'static Vec<Handle<$ty>>, 
            random get.get_random().clone_weak() Handle<$ty> 
        |$name);)*)*
    };
    (@Vec #load $($ty:ty: $($name: ident),*;)*) => {
        $($(type_cell!(on $ty > store HandleVec<$ty> | 
            set.into() HandleVecLoad | 
            get.get_vec().get(id:usize).unwrap().clone() Handle<$ty>, 
            vec get.get_vec() &'static Vec<Handle<$ty>>
        |$name);)*)*
    };
    (@Vec #load #random $($ty:ty: $($name: ident),*;)*) => {
        $($(type_cell!(on $ty > store HandleVec<$ty> | 
            set.into() HandleVecLoad | 
            get.get_vec().get(id:usize).unwrap().clone() Handle<$ty>, 
            vec get.get_vec() &'static Vec<Handle<$ty>>, 
            random get.get_vec().get_random().clone_weak() Handle<$ty> 
        |$name);)*)*
    };

    (@HashMap<$id:ty> $($ty:ty: $($name: ident),*;)*) => {type_cell!{@HashMap<$id> #unwrap #clone $($ty>Handle<$ty>:$($name),*;)*}};
}

pub trait BevyCellVecRandom <T> {
    fn get_vec(&self) -> &Vec<T>;
    fn get_random(&self) -> &T;
}
impl <T> BevyCellVecRandom <T> for Vec<T> {
    fn get_vec(&self) -> &Vec<T> {self}
    fn get_random(&self) -> &T {
        let v = self.get_vec();
        let i = rand::thread_rng().gen_range(0..v.len());
        &v[i]
    }
}

#[derive(Debug)]
pub struct HandleVec <T: Asset> (pub Vec<Handle<T>>);
impl <T: Asset> HandleVec <T> {
    pub fn get_vec(&self) -> &Vec<Handle<T>> {&self.0}
    pub fn load(load: HandleVecLoad) -> Self {
        let mut v = vec![];
        for i in load.range {
            v.push(load.assets.load(format!("{}_{:03}.{}",load.name,i,load.ext)));
        }
        HandleVec(v)
    }
}

#[derive(Intuple,new)]
pub struct HandleVecLoad <'a>{
    assets:&'a Res<'a, AssetServer>,
    name:&'a str,
    range:RangeInclusive<i32>,
    ext:&'a str
}

impl<'a,T: Asset> From<HandleVecLoad<'a>> for HandleVec<T> {
    fn from(value: HandleVecLoad<'a>) -> Self {
        HandleVec::load(value)
    }
}