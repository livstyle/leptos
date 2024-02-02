use super::{SignalReadGuard, SignalUntrackedWriteGuard, SignalWriteGuard};
use crate::{
    graph::{
        AnySource, AnySubscriber, ReactiveNode, Source, SubscriberSet,
        ToAnySource,
    },
    prelude::{IsDisposed, Trigger},
    traits::{DefinedAt, Readable, Writeable},
};
use core::fmt::{Debug, Formatter, Result};
use std::{
    panic::Location,
    sync::{Arc, RwLock, Weak},
};

pub struct ArcRwSignal<T> {
    #[cfg(debug_assertions)]
    defined_at: &'static Location<'static>,
    pub(crate) value: Arc<RwLock<T>>,
    inner: Arc<RwLock<SubscriberSet>>,
}

impl<T> Clone for ArcRwSignal<T> {
    #[track_caller]
    fn clone(&self) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: self.defined_at,
            value: Arc::clone(&self.value),
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Debug for ArcRwSignal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ArcRwSignal")
            .field("type", &std::any::type_name::<T>())
            .field("value", &Arc::as_ptr(&self.value))
            .finish()
    }
}

impl<T> ArcRwSignal<T> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(level = "trace", skip_all,)
    )]
    pub fn new(value: T) -> Self {
        Self {
            #[cfg(debug_assertions)]
            defined_at: Location::caller(),
            value: Arc::new(RwLock::new(value)),
            inner: Arc::new(RwLock::new(SubscriberSet::new())),
        }
    }
}

impl<T> DefinedAt for ArcRwSignal<T> {
    #[inline(always)]
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        #[cfg(debug_assertions)]
        {
            Some(self.defined_at)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }
}

impl<T> IsDisposed for ArcRwSignal<T> {
    #[inline(always)]
    fn is_disposed(&self) -> bool {
        false
    }
}

impl<T> Readable for ArcRwSignal<T> {
    type Value = T;

    fn try_read(&self) -> Option<SignalReadGuard<'_, T>> {
        self.value.read().ok().map(SignalReadGuard::from)
    }
}

impl<T> Trigger for ArcRwSignal<T> {
    fn trigger(&self) {
        self.mark_dirty();
    }
}

impl<T> Writeable for ArcRwSignal<T> {
    type Value = T;

    fn try_write(&self) -> Option<SignalWriteGuard<'_, Self, Self::Value>> {
        self.value
            .write()
            .ok()
            .map(|guard| SignalWriteGuard::new(self, guard))
    }

    fn try_write_untracked(
        &self,
    ) -> Option<SignalUntrackedWriteGuard<'_, Self::Value>> {
        self.value.write().ok().map(SignalUntrackedWriteGuard::from)
    }
}

impl ReactiveNode for RwLock<SubscriberSet> {
    fn mark_dirty(&self) {
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        for sub in self.write().unwrap().take() {
            sub.mark_check();
        }
    }

    fn update_if_necessary(&self) -> bool {
        // if they're being checked, signals always count as "dirty"
        true
    }
}

impl Source for RwLock<SubscriberSet> {
    fn clear_subscribers(&self) {
        self.write().unwrap().take();
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.write().unwrap().subscribe(subscriber)
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.write().unwrap().unsubscribe(subscriber)
    }
}

impl<T> ReactiveNode for ArcRwSignal<T> {
    fn mark_dirty(&self) {
        self.mark_subscribers_check();
    }

    fn mark_check(&self) {}

    fn mark_subscribers_check(&self) {
        self.inner.mark_subscribers_check();
    }

    fn update_if_necessary(&self) -> bool {
        // if they're being checked, signals always count as "dirty"
        true
    }
}

impl<T> Source for ArcRwSignal<T> {
    fn clear_subscribers(&self) {
        self.inner.clear_subscribers();
    }

    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.inner.add_subscriber(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.inner.remove_subscriber(subscriber);
    }
}

impl<T> ToAnySource for ArcRwSignal<T> {
    fn to_any_source(&self) -> AnySource {
        AnySource(
            Arc::as_ptr(&self.inner) as usize,
            Arc::downgrade(&self.inner) as Weak<dyn Source + Send + Sync>,
        )
    }
}
