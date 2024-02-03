//! An implementation of a fine-grained reactive system.
//!
//! Fine-grained reactivity is an approach to modeling the flow of data through an interactive
//! application by composing together three categories of reactive primitives:
//! 1. **Signals**: atomic units of state, which can be directly mutated.
//! 2. **Computations**: derived values, which cannot be mutated directly but update whenever the signals
//!    they depend on change. These include both synchronous and asynchronous derived values.
//! 3. **Effects**: side effects that synchronize the reactive system with the non-reactive world
//!    outside it.
//!
//! The state of an entire application can be modeled as a reactive graph of this kind.

use futures::Stream;
use std::{
    future::Future,
    pin::Pin,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

pub(crate) mod channel;
pub mod computed;
pub mod effect;
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
