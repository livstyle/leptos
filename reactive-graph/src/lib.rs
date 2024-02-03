use futures::Stream;
use std::{
    future::Future,
    pin::Pin,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

pub(crate) mod channel;
//pub mod effect;
pub mod executor;
pub mod graph;
pub mod owner;
pub mod signal;
pub mod traits;

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedLocalFuture<T> = Pin<Box<dyn Future<Output = T>>>;
pub type PinnedStream<T> = Pin<Box<dyn Stream<Item = T> + Send + Sync>>;

pub mod prelude {
    pub use crate::traits::*;
}

trait OrPoisoned {
    type Inner;

    fn or_poisoned(self) -> Self::Inner;
}

impl<'a, T> OrPoisoned
    for Result<RwLockReadGuard<'a, T>, PoisonError<RwLockReadGuard<'a, T>>>
{
    type Inner = RwLockReadGuard<'a, T>;

    fn or_poisoned(self) -> Self::Inner {
        self.expect("lock poisoned")
    }
}

impl<'a, T> OrPoisoned
    for Result<RwLockWriteGuard<'a, T>, PoisonError<RwLockWriteGuard<'a, T>>>
{
    type Inner = RwLockWriteGuard<'a, T>;

    fn or_poisoned(self) -> Self::Inner {
        self.expect("lock poisoned")
    }
}
