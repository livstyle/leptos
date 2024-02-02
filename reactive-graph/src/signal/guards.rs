use crate::traits::Trigger;
use std::{
    ops::{Deref, DerefMut},
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug)]
pub struct SignalReadGuard<'a, T>(RwLockReadGuard<'a, T>);

impl<'a, T> From<RwLockReadGuard<'a, T>> for SignalReadGuard<'a, T> {
    fn from(value: RwLockReadGuard<'a, T>) -> Self {
        SignalReadGuard(value)
    }
}

impl<'a, T> Deref for SignalReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct SignalWriteGuard<'a, S, T>
where
    S: Trigger,
{
    triggerable: &'a S,
    guard: Option<RwLockWriteGuard<'a, T>>,
}

impl<'a, S, T> SignalWriteGuard<'a, S, T>
where
    S: Trigger,
{
    pub fn new(triggerable: &'a S, guard: RwLockWriteGuard<'a, T>) -> Self {
        Self {
            guard: Some(guard),
            triggerable,
        }
    }
}

impl<'a, S, T> Deref for SignalWriteGuard<'a, S, T>
where
    S: Trigger,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard
            .as_ref()
            .expect(
                "the guard should always be in place until the Drop \
                 implementation",
            )
            .deref()
    }
}

impl<'a, S, T> DerefMut for SignalWriteGuard<'a, S, T>
where
    S: Trigger,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard
            .as_mut()
            .expect(
                "the guard should always be in place until the Drop \
                 implementation",
            )
            .deref_mut()
    }
}

#[derive(Debug)]
pub struct SignalUntrackedWriteGuard<'a, T>(RwLockWriteGuard<'a, T>);

impl<'a, T> From<RwLockWriteGuard<'a, T>> for SignalUntrackedWriteGuard<'a, T> {
    fn from(value: RwLockWriteGuard<'a, T>) -> Self {
        Self(value)
    }
}

impl<'a, T> Deref for SignalUntrackedWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, T> DerefMut for SignalUntrackedWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

// Dropping the write guard will notify dependencies.
impl<'a, S, T> Drop for SignalWriteGuard<'a, S, T>
where
    S: Trigger,
{
    fn drop(&mut self) {
        // first, drop the inner guard
        drop(self.guard.take());

        // then, notify about a change
        self.triggerable.trigger();
    }
}
