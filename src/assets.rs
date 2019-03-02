//! Integration of ggez types with warmy resource loader

use ggez::{self, graphics};
use warmy::{SimpleKey, Storage, Loaded};

use failure::{self, Fail};
//use crate::error::*;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    CannotLoadFromLogical,
    GameError(ggez::GameError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::CannotLoadFromLogical=> f.write_str("cannot load from Logical"),
            Error::GameError(ref e) => write!(f, "GGEZ error: {}", e),
        }
    }
}

/// A wrapper for a ggez Image, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);

impl warmy::Load<ggez::Context, SimpleKey> for Image {
    type Error = Error;

    fn load(
        key: SimpleKey,
        storage: &mut Storage<ggez::Context, SimpleKey>,
        ctx: &mut ggez::Context
    ) -> Result<Loaded<Self, SimpleKey>, Self::Error> {
       match key {
           SimpleKey::Path(path) => {
               //println!("key: {:?}, root: {:?}, path: {:?}", &key, storage.root(), path);
               debug!("Loading image {:?} from file {:?}", path, storage.root());

               graphics::Image::new(ctx, path)
                    .map(|x| Image(x).into())
                    .map_err(|e| Error::GameError(e))
           },
           SimpleKey::Logical(_) => Err(Error::CannotLoadFromLogical)
       }
    }
}
