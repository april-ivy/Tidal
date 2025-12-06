mod albums;
mod artists;
mod client;
mod discovery;
mod favorites;
pub(crate) mod models;
mod playback;
mod playlists;
mod search;
mod tracks;
mod users;

pub use client::{
    ClientConfig,
    TidalClient,
};
pub use models::*;
