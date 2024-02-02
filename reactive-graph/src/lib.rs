use futures::Stream;
use std::{future::Future, pin::Pin};

pub mod graph;
pub mod signal;
pub mod traits;

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedStream<T> = Pin<Box<dyn Stream<Item = T> + Send + Sync>>;

pub mod prelude {
    pub use crate::traits::*;
}


