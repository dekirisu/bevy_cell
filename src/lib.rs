use hashbrown::HashMap;
pub use type_cell::*;
pub type TyMap<K,V> = HashMap<K,V>;

#[macro_export]
macro_rules! bycell {
    // Handles
    (=$on:ty: [$o:ty|$($name:tt)*];)=>{paste!{
        tycell!{ $on > Handle<$o>: [$($name)*.clone_weak()]; }
    }}; 
    (=$on:ty: [|$($name:tt)*];)=>{paste!{
        tycell!{ !Handle<$on>: [$($name)*.clone_weak()]; }
    }}; 
    // Raw
    (=$on:ty: [$o:ty:$($name:tt)*];)=>{paste!{
        tycell!{ $on > $o: [$($name)*]; }
    }}; 
    (=$on:ty: [:$($name:tt)*];)=>{paste!{
        tycell!{ $on: [$($name)*]; }
    }}; 
    // Entities
    (=$on:ty: [$($name:tt)*];)=>{paste!{
        tycell!{ $on > Entity: [$($name)*.clone()]; }
    }}; 
    // Merge
    ($( $on:ty: $([$($name:tt)*])*;)*)=>{paste!{
        $($(bycell!{= $on: [$($name)*]; })*)*
    }}; 
    ($( {$on:ty} $([$($name:tt)*])*)*)=>{paste!{
        $($(bycell!{= $on: [$($name)*]; })*)*
    }}; 
}