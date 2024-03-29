use ggez_goodies::scene;

use crate::input;

use crate::world::World;

pub mod level;

// Shortcuts for our scene type
pub type FSceneSwitch = scene::SceneSwitch<World, input::InputEvent>;
pub type FSceneStack = scene::SceneStack<World, input::InputEvent>;
