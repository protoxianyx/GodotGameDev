use godot::prelude::*;
// use godot::engine::Sprite2D;




struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

// #[derive(GodotClass)]
// #[class(base=Sprite2D)]
// struct PlayerOne {
// 	speed: f64,
// 	angular_speed: f64,
// 	base: Base<Sprite2D>
// }