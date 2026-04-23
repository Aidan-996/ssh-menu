//! Configuration module.
//!
//! Handles loading, saving, and data modeling of `~/.ssh-menu.toml`.
//! Submodules:
//! - [`model`]: `Config`, `Host` structs + display/search helpers.
//! - [`store`]: load / save / resolve-path I/O.

pub mod model;
pub mod store;

pub use model::{Config, Host};
pub use store::{default_config_path, load, save};
