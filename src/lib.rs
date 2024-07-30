#![warn(clippy::correctness, clippy::perf, clippy::suspicious, clippy::complexity, clippy::nursery, clippy::cargo, clippy::style)]

pub mod event;
pub mod subscriber;
pub mod bus;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
}