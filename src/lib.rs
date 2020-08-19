#[cfg(feature = "mongo")] #[macro_use] extern crate bson;
#[macro_use] pub mod query;
#[cfg(feature = "mongo")] pub mod mongo;
