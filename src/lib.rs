pub mod event;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
}