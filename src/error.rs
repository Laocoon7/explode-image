use crate::prelude::*;

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Calculated tile size is not a round number. ({}, {})", .0, .1)]
    InvalidTileSize(f32, f32),

    #[error(transparent)]
    ImageError(#[from] ImageError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
