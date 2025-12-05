#![allow(dead_code, unused_imports)]

pub mod api;
pub mod auth;
pub mod decrypt;
pub mod lyrics;
pub mod stream;

pub use api::*;
pub use auth::*;
pub use decrypt::*;
pub use lyrics::*;
pub use stream::*;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
