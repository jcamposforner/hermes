#![warn(clippy::all)]

pub mod event;
pub mod subscriber;
pub mod bus;

#[cfg(feature = "serializer")]
pub mod serializer;

#[cfg(feature = "rabbit")]
#[cfg(feature = "async")]
pub mod consumer;

#[cfg(feature = "rabbit")]
pub mod rabbit;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
    pub use hermes_derive::EventMetadata;
}
