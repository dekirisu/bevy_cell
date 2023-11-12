<p align="center">
    <img src="https://user-images.githubusercontent.com/78398528/282285927-6f6c28a4-7d52-46ab-b29d-1d43cbc96374.gif">
</p>
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

🦊 Easily attach <a href="https://github.com/bevyengine/bevy">bevy</a>'s Handles/Entities statically to types <br>
🐑 Get them in any system, without using Resources.<br>
🦄 Basically just simplified <a href="https://github.com/dekirisu/type_cell">type_cell</a> for bevy!
```toml
[dependencies]
# Should work for any bevy version, as long as Entity/Handles don't change
bevy_cell = "0.12"
```
```rust 
use bevy_cell::*;
// setup cells
handles!{Image: cat, dog;}
entities!{player}
// on startup
Image::set_cat(img_handle);
Entity::set_player(entity);
// in any system
Image::cat();
Entity::player();
```
## Usage
🐁 The macro also accepts multiple definitions at once. <br>
🦎 Wrapping the names in parenthesis changes their type:
* `getter`: `Handle<T>/Entity`
    * getter() returns a weak_clone of the Handle / a reference o the Entity
* `[getter]`: `Vec<Handle<T>/Entity>`
    * getter(id) - see above
    * getter_vec() - returns a mut reference to the Vec - use with care: read below!
* `{getter:K}`: `HashMap<K,Handle<T>/Enitity>`
    * getter(id) - see above
    * getter_map() - returns a mut reference to the HashMap - use with care: read below!
```rust 
handles!{
    Image: single_img, [vec_imgs], {hashmap_imgs:u32};
    Mesh: mesh_a, mesh_b;
}
entities!{
    entity_a, entity_b, [entities]
}
Image::set_vec_imgs([Image::_]);
let img = Image::vec_imgs(0);
```
🐕 You can change the type on which the values will be attached on by using `getter@Type`
```rust 
handles!{
    Image: single_img@Scene, [vec_imgs@Mesh], {hashmap_imgs@Scene:u32};
    Mesh: mesh_a@Scene, mesh_b@Image;
}
entities!{
    entity_a, entity_b, [entities]
}
Mesh::set_vec_imgs([Image::_]);
let img = Mesh::vec_imgs(0);
```
## Mutability
🐝 Getting the mutable references to Vecs and HashMaps bypasses rusts safety!<br>
🦞 Therefore it's important to be sure not to mutate them while it's read anywhere else<br>
🦊 Here are 2 use cases in which controlling it is easy:
* mutate/set on Startup
* mutate/set on State Enter/Exits

🐍 Use bevy Resources for anything else!
## Disclaimer
🐢 I made this crate cause I disliked to have many resources as system parameters. <br>
🦎 It's a bit hacky! Just use bevy Resources if you're not sure!<br>
🐅 Use <a href="https://github.com/dekirisu/type_cell">type_cell</a> if you just dislike the way values are defined!

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>