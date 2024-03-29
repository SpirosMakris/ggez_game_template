//! This file defines the `World`,
//! as well as some handy utility methods and structs.
//! The `World` contains shared state that will be available
//! to every `Scene`: specs objects, input state, asset cache.


use ggez;
use ggez::nalgebra::{Point2, Vector2};
use ggez_goodies::input as ginput;
use specs;
use specs::Builder;

use warmy;

use std::path;

use crate::components::*;
use crate::input;

pub struct World {
    pub assets: warmy::Store<ggez::Context, warmy::SimpleKey>,
    pub input: input::InputState,
    pub specs_world: specs::World,
}

impl World {
    fn register_components(&mut self) {
        self.specs_world.register::<Position>();
        self.specs_world.register::<Motion>();
        self.specs_world.register::<Shot>();
        self.specs_world.register::<Player>();
    }

    pub fn new(ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
        // We bridge the gap between ggez and warmy path handling here; ggez assumes its own absolute paths, warmy assumes system-absolute
        // paths; so, we make warmy look in the specified resource dir
        // (normally $CARGO_MANIFEST_DIR/assets) or the ggez default asset dir.
        let asset_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ggez::filesystem::resources_dir(ctx).to_owned()
        };

        info!("Setting up resource path in world: {:?}", asset_pathbuf);

        let store_opt = warmy::StoreOpt::default().set_root(asset_pathbuf);
        let store = warmy::Store::new(store_opt)
            .expect("Could not create asset store! Does the directory exist?");
       
        let w = specs::World::new();

        let mut the_world = Self {
            assets: store,
            input: ginput::InputState::new(),
            specs_world: w,
        };

        the_world.register_components();
        
        // @TODO: -- Remove this
        // Make a test entity
        the_world
            .specs_world
            .create_entity()
            .with(Position(Point2::new(0.0, 0.0)))
            .with(Motion {
                velocity: Vector2::new(1.0, 1.0),
                acceleration: Vector2::new(0.0, 0.0),
            })
            .build();

        
        the_world
    }
}

