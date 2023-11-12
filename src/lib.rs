pub use type_cell::*;
pub use type_cell::paste::paste;

#[macro_export]
macro_rules! entities {
    ($($name:tt),*) => {
        $(entities!{=$name})*
    };
    (=$name:ident) => {entities!{=($name@Entity)}};
    (=($name:ident@$on:ty)) => {paste!{
        type_cell! { $on {
            static Entity: risky!
            set [<set_ $name>]();
            get $name();
        }}
    }};
    (=[$name:ident]) => {entities!{=[$name@Entity]}};
    (=[$name:ident@$on:ty]) => {paste!{
        type_cell! { $on {
            static Vec<Entity>: risky!
            set [<set_ $name>]();
            get [<$name _vec>]();
            get $name() -> &'static Entity: static.get(id:usize).unwrap();
        }}
    }};
    (={$name:ident:$ty:ty}) => {entities!{={$name@Entity:$ty}}};
    (={$name:ident@$on:ty:$ty:ty}) => {paste!{
        type_cell! { $on {
            static HashMap<$ty,Entity>: risky!
            set [<set_ $name>]();
            get [<$name _map>]();
            get $name() -> &'static Entity: static.get(id:&$ty).unwrap();
        }}
    }};
}

#[macro_export]
macro_rules! handles {
    ($($mt:ty:$($name:tt),*;)*) => {
        $($(handles!{=$mt|$name})*)*
    };    
    (=$mt:ty|$name:ident) => {handles!{=$mt|($name@$mt)}};
    (=$mt:ty|($name:ident@$on:ty)) => {paste!{
        type_cell! { $on {
            static Handle<$mt>: risky!
            set [<set_ $name>]();
            get $name() -> Handle<$mt>: static.clone_weak();
        }}
    }};
    (=$mt:ty|[$name:ident]) => {handles!{=$mt|[$name@$mt]}};
    (=$mt:ty|[$name:ident@$on:ty]) => {paste!{
        type_cell! { $on {
            static Vec<Handle<$mt>>: risky!
            set [<set_ $name>]();
            get [<$name _vec>]();
            get $name() -> Handle<$mt>: static.get(id:usize).unwrap().clone_weak();
        }}
    }};
    (=$mt:ty|{$name:ident:$ty:ty}) => {handles!{=$mt|{$name@$mt:$ty}}};
    (=$mt:ty|{$name:ident@$on:ty:$ty:ty}) => {paste!{
        type_cell! { $on {
            static HashMap<$ty,Handle<$mt>>: risky!
            set [<set_ $name>]();
            get [<$name _map>]();
            get $name() -> Handle<$mt>: static.get(id:&$ty).unwrap().clone_weak();
        }}
    }};
}