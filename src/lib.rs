//! Biblioteca raiz do Axolote Suite.

pub mod app;
pub mod core;
pub mod modules;
pub mod shared;

pub use app::{AxoloteSuite, Message, Module};
