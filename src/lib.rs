#![warn(clippy::all)]

pub mod event;
pub mod subscriber;
pub mod bus;

#[cfg(feature = "serializer")]
pub mod serializer;
mod consumer;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
}
