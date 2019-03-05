use ggez::graphics::*;
use specs::*;
use specs::storage::*;
use ggez::nalgebra as na;

/// ////////////////////////////////
/// Components
/// ////////////////////////////////
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub na::Point2<f32>);

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: na::Vector2<f32>,
    pub acceleration: na::Vector2<f32>, }

// Ust a marker that a particular entity is the player
#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Shot {
    pub damage: u32,
}

#[derive(Clone, Debug, Component)]
#[storage(HashMapStorage)]
pub struct CBackgroundScroller {
    pub scroll_speed: na::Vector2<f32>,
}

impl CBackgroundScroller {
    pub fn new() -> Self {
        CBackgroundScroller { scroll_speed: na::Vector2::new(0.0, -0.01) }
    }
}

