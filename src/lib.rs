use hashbrown::HashMap;
pub use type_cell::*;
pub type TyMap<K,V> = HashMap<K,V>;

#[macro_export]
macro_rules! bycell {
    (=$on:ty: [$o:ty|$($name:tt)*];)=>{paste!{
        tycell!{ $on > Handle<$o>: [$($name)*.clone_weak()]; }
    }}; 
    (=$on:ty: [|$($name:tt)*];)=>{paste!{
        tycell!{ !Handle<$on>: [$($name)*.clone_weak()]; }
    }}; 
    (=$on:ty: [$($name:tt)*];)=>{paste!{
        tycell!{ $on > Entity: [$($name)*.clone()]; }
    }}; 
    ($( $on:ty: $([$($name:tt)*])*;)*)=>{paste!{
        $($(bycell!{= $on: [$($name)*]; })*)*
    }}; 
    ($( {$on:ty} $([$($name:tt)*])*)*)=>{paste!{
        $($(bycell!{= $on: [$($name)*]; })*)*
    }}; 
}