#![warn(clippy::all)]

pub mod event;
pub mod subscriber;
pub mod bus;

#[cfg(feature = "serializer")]
pub mod serializer;
pub mod consumer;

#[cfg(feature = "rabbit")]
pub mod rabbit;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
}
