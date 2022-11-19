use error::MyError;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

pub type Result<T, E = MyError> = core::result::Result<T, E>;
