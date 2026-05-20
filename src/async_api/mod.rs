//! Executor-agnostic async stream wrappers for `TKTokenWatcher` callbacks.
//!
//! Enabled with the `async` Cargo feature.
//!
//! ## Wrapped APIs
//!
//! - [`AsyncTokenWatcher::insertion_stream`] wraps `TKTokenWatcher.setInsertionHandler`.
//! - [`AsyncTokenWatcher::removal_stream`] wraps `TKTokenWatcher.addRemovalHandler`.
//!
//! ## Removal-handler note
//!
//! `TKTokenWatcher` does not expose removal-handler unregistration. Dropping a
//! [`TokenRemovalStream`] drops the async consumer, but the underlying removal
//! observer stays registered until the owning [`AsyncTokenWatcher`] is dropped.

#![cfg(feature = "async")]

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use doom_fish_utils::stream::{BoundedAsyncStream, NextItem};

use crate::error::CryptoTokenKitError;
use crate::token_watcher::{TokenWatcher, TokenWatcherTokenInfo};

fn borrow_watcher(watcher: &Rc<RefCell<TokenWatcher>>) -> Ref<'_, TokenWatcher> {
    watcher.borrow()
}

fn borrow_watcher_mut(watcher: &Rc<RefCell<TokenWatcher>>) -> RefMut<'_, TokenWatcher> {
    watcher.borrow_mut()
}

/// Shared async owner for `TKTokenWatcher` callback subscriptions.
#[derive(Clone)]
pub struct AsyncTokenWatcher {
    watcher: Rc<RefCell<TokenWatcher>>,
}

impl core::fmt::Debug for AsyncTokenWatcher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AsyncTokenWatcher").finish_non_exhaustive()
    }
}

impl AsyncTokenWatcher {
    /// Create a new async token watcher wrapper.
    #[must_use]
    pub fn new() -> Self {
        Self {
            watcher: Rc::new(RefCell::new(TokenWatcher::new())),
        }
    }

    /// Snapshot the currently visible token identifiers.
    pub fn token_ids(&self) -> Result<Vec<String>, CryptoTokenKitError> {
        borrow_watcher(&self.watcher).token_ids()
    }

    /// Snapshot token metadata for a visible token identifier.
    pub fn token_info(
        &self,
        token_id: &str,
    ) -> Result<Option<TokenWatcherTokenInfo>, CryptoTokenKitError> {
        borrow_watcher(&self.watcher).token_info(token_id)
    }

    /// Subscribe to token-insertion events.
    pub fn insertion_stream(
        &self,
        capacity: usize,
    ) -> Result<TokenInsertionStream, CryptoTokenKitError> {
        TokenInsertionStream::subscribe(Rc::clone(&self.watcher), capacity)
    }

    /// Subscribe to removal events for a specific token identifier.
    pub fn removal_stream(
        &self,
        token_id: &str,
        capacity: usize,
    ) -> Result<TokenRemovalStream, CryptoTokenKitError> {
        TokenRemovalStream::subscribe(Rc::clone(&self.watcher), token_id, capacity)
    }
}

impl Default for AsyncTokenWatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Async stream of inserted token identifiers.
pub struct TokenInsertionStream {
    inner: BoundedAsyncStream<String>,
    watcher: Rc<RefCell<TokenWatcher>>,
}

impl core::fmt::Debug for TokenInsertionStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TokenInsertionStream")
            .field("buffered", &self.buffered_count())
            .finish_non_exhaustive()
    }
}

impl TokenInsertionStream {
    fn subscribe(
        watcher: Rc<RefCell<TokenWatcher>>,
        capacity: usize,
    ) -> Result<Self, CryptoTokenKitError> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        borrow_watcher_mut(&watcher).set_insertion_handler(move |token_id| {
            sender.push(token_id);
        })?;
        Ok(Self {
            inner: stream,
            watcher,
        })
    }

    /// Wait asynchronously for the next inserted token identifier.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, String> {
        self.inner.next()
    }

    /// Try to retrieve the next buffered insertion without blocking.
    #[must_use]
    pub fn try_next(&self) -> Option<String> {
        self.inner.try_next()
    }

    /// Number of buffered insertion events.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Snapshot the currently visible token identifiers.
    pub fn token_ids(&self) -> Result<Vec<String>, CryptoTokenKitError> {
        borrow_watcher(&self.watcher).token_ids()
    }

    /// Snapshot token metadata for a visible token identifier.
    pub fn token_info(
        &self,
        token_id: &str,
    ) -> Result<Option<TokenWatcherTokenInfo>, CryptoTokenKitError> {
        borrow_watcher(&self.watcher).token_info(token_id)
    }
}

impl Drop for TokenInsertionStream {
    fn drop(&mut self) {
        let _ = borrow_watcher_mut(&self.watcher).set_insertion_handler(|_| {});
    }
}

/// Async stream of removal events for one watched token identifier.
pub struct TokenRemovalStream {
    inner: BoundedAsyncStream<String>,
    watcher: Rc<RefCell<TokenWatcher>>,
    token_id: String,
}

impl core::fmt::Debug for TokenRemovalStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TokenRemovalStream")
            .field("token_id", &self.token_id)
            .field("buffered", &self.buffered_count())
            .finish_non_exhaustive()
    }
}

impl TokenRemovalStream {
    fn subscribe(
        watcher: Rc<RefCell<TokenWatcher>>,
        token_id: &str,
        capacity: usize,
    ) -> Result<Self, CryptoTokenKitError> {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        borrow_watcher_mut(&watcher).add_removal_handler(token_id, move |removed_token_id| {
            sender.push(removed_token_id);
        })?;
        Ok(Self {
            inner: stream,
            watcher,
            token_id: token_id.to_owned(),
        })
    }

    /// Token identifier registered with `TKTokenWatcher.addRemovalHandler`.
    #[must_use]
    pub fn watched_token_id(&self) -> &str {
        &self.token_id
    }

    /// Wait asynchronously for the next removal event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, String> {
        self.inner.next()
    }

    /// Try to retrieve the next buffered removal without blocking.
    #[must_use]
    pub fn try_next(&self) -> Option<String> {
        self.inner.try_next()
    }

    /// Number of buffered removal events.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Snapshot token metadata for a visible token identifier.
    pub fn token_info(
        &self,
        token_id: &str,
    ) -> Result<Option<TokenWatcherTokenInfo>, CryptoTokenKitError> {
        borrow_watcher(&self.watcher).token_info(token_id)
    }
}
