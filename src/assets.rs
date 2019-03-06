//! Integration of ggez types with warmy resource loader

use ggez::{self, graphics};
use warmy::{SimpleKey, Storage, Loaded};

use failure::{self, Fail};
//use crate::error::*;

use std::fmt;
use std::path;

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

/// Warmy hands our `load()` method an absolute path, while ggez takes absolute
/// paths into its VFS directory.  Warmy needs to know the real absolute path so
/// it can watch for reloads, so this function strips the path prefix of the warmy
/// Store's root off of the given absolute path and turns it into the style of path
/// that ggez expects.
///
/// Because of this, ggez will have several places that resources *may* live but
/// warmy will only watch for reloads in one of them.  However, that isn't a huge
/// problem: for development you generally want all your assets in one place to
/// begin with, and for deployment you don't need the hotloading functionality.
///
/// TODO: With warmy 0.7 this should not be necessary, figure it out.
fn warmy_to_ggez_path(path: &path::Path, root: &path::Path) -> path::PathBuf {
    let stripped_path = path.strip_prefix(root)
        .expect("warmy path is outside of the warmy store? Should never happen.");
    path::Path::new("/").join(stripped_path)
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
               debug!("Loading image {:?} from file {:?}", path, path);

               let path = warmy_to_ggez_path(path.as_path(), storage.root());

               graphics::Image::new(ctx, path)
                    .map(|x| Image(x).into())
                    .map_err(|e| Error::GameError(e))
           },
           SimpleKey::Logical(_) => Err(Error::CannotLoadFromLogical)
       }
    }
}
