//! ```rust
//! fn thing() {
//!
//! }
//! ```
//!
//! ->
//!
//! ```
//! fn thing() {
//!
//! fn inner() {
//!
//! }
//!
//! inner().await
//!
//! }
//! ```
//!

#![no_std]

// pub struct ProxySpawner<T> {
//     data: T,
//     rec: Receiver<dyn Future<Output = ()>>,
// }

// impl<T> ProxySpawner<T> {
//     pub fn guard(&self) -> ProxySpawnerGuard<T> {
//         ProxySpawnerGuard { spawner: self }
//     }
// }

// pub struct ProxySpawnerGuard<'spawner, T> {
//     spawner: &'spawner ProxySpawner<T>,
// }

use core::{
    pin::Pin,
    task::{Context, Poll},
};

mod oneshot;

/// Wraps future to return nothing and send result over a channel
enum ProxyFuture<F: Future + Sync> {
    Unpolled {
        inner: F,
        spawner: fn(&dyn Future<Output = ()>),
    },
    Polled {
        inner: F,
        channel: RwL<Option<F::Output>>,
    },
}

impl<F: Future + Sync> Future for ProxyFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };

        match this {
            ProxyFuture::Unpolled { inner, spawner } => spawner(),
            ProxyFuture::Polled { inner } => todo!(), // check channel
        }
        // inner.poll(cx)
    }
}

impl Future for &dyn ProxyFuture {}

struct ProxyFutureRef<'future> {
    reference: &'future ProxyFuture,
}

impl Future for ProxyFutureRef {}

// enum ProxyFuture<Fut, Output>
// where
//     Fut: Future<Output = Output> + Send,
// {
//     Unpolled { inner: Fut, sender: Sender<Fut> },
//     Polled { res: Receiver<Output> },
// }

// impl<Fut, Output> Future for ProxyFuture<Fut, Output>
// where
//     Fut: Future<Output = Output> + Send,
// {
//     type Output = Output;

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         let inner = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };
//         inner.poll(cx)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
}
