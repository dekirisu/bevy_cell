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
🦄 See <a href="https://github.com/dekirisu/type_cell">type_cell</a> for any other use case!
<br>
```toml
[dependencies]
bevy_cell = "0.13"
```
```rust 
use bevy_cell::*;
```
---

<b><u>I.</u> There are 2 valid syntaxes:</b><br>
🐰 `{Type} [name1] [name2] [name3]`<br>
🦝 `Type: [name1] [name2] [name3];`
<br><br>
<b>u>II.</u> The syntax inside the `[]` will change the attached type:</b><br>
🐈 **Entity** - Just choose a name: `[camera]`<br>
🦥 **Handle** - Its type separated by a `|`:  `[Image|cat]`<br>
🐒 If no type is set, the parent type is used: `[|cat]`
<br><br>
<b><u>III.</u> Setting the collection type is also done inside `[]`:</b><br>
🦄 **Single** - Using the syntax as in <u>**II.**</u><br>
🐔 **Vec** - add a `<>` after the name: `[cameras<>]`<br>
🐲 **HashMap** - add a `<KeyType>` after the name: `[cameras<usize>]`
<br>
```rust 
// Macro Examples
bycell! {
    Camera: [main] [secondary];
    AudioSource: [|hit] [|shots<>];
    Player: [main] [Scene|models<u8>];
}
```
<b><u>IV.</u> Setting Values:</b><br>
🐑 Use `Type::set_..(value)` **ONCE** on (pre-)startup<br>
🦌 The value can be anything implementing its type!
```rust 
// Setter Examples
Camera::set_main(commands.spawn(..).id());
AudioSource::set_shots([
    assets.load("shot0.ogg"),
    assets.load("shot1.ogg"),
    assets.load("shot3.ogg"),
]);
Player::set_models([
    (5, assets.load("player5.glb")),
    (7, assets.load("player7.glb")),
]);
```
<b><u>V.</u> Getting Values:</b><br>
🐏 Different getters are provided, depending on the collection type!
```rust 
// Single Getter
Camera::main();            // Cloned
Camera::main_ref();        // Static Reference
// Vec Getters
AudioSource::shots(1);     // Cloned
AudioSource::shots_ref(1); // Static Reference
AudioSource::shots_vec();  // Static Reference to Vec
// HashMap Getters
Player::models(&5);        // Cloned
Player::models_ref(&5);    // Static Reference
Player::models_map();      // Static Reference to HashMap
```

<b><u>VI.</u> Mutability:</b><br>
🐝 You can make any of those mutable by adding a `mut` before the name<br>
🦞 Only use this if you can avoid race conditions<br>
🦧 One idea is to mutate something on state change!
```rust 
// Macro Examples
bycell! {
    Camera: [mut main] [mut secondary];
    AudioSource: [|mut hit] [|mut shots<>];
    Player: [mut main] [Scene|mut models<u8>];
}
```
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