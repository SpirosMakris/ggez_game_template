use ::ggez;
use ggez::graphics;
use ggez_goodies::scene;
use specs::{self, Join};
use ::warmy;
use std::path::Path;

use crate::components as c;
use crate::input;
use crate::assets;
use crate::scenes::*;
use crate::systems::*;
use crate::world::World;

use ::ggez::nalgebra::Point2;

pub struct LevelScene {
    done: bool,
    kiwi: ::warmy::Res<assets::Image>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let kiwi = world
            .assets
            .get::<assets::Image>(&Path::new("/images/kiwi.png").into(), ctx)
            .unwrap();

        let dispatcher = Self::register_systems();
        LevelScene {
            done,
            kiwi,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
            .build()
    }
}

impl scene::Scene<World, input::InputEvent> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> FSceneSwitch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        for p in pos.join() {
            //let dst = Point2::new(p.0, 0.0);
            graphics::draw(ctx, &(self.kiwi.borrow().0), (p.0,))?;
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Level Scene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::InputEvent, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}


