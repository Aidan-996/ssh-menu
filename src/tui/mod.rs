//! Interactive terminal UI, built on `ratatui` + `crossterm`.
//!
//! Submodules:
//! - [`app`]: application state machine (mode, filtered list, exit intent).
//! - [`form`]: add/edit host form state.
//! - [`events`]: keyboard event dispatch.
//! - [`view`]: rendering (list + header + footer + form overlay).
//! - [`runtime`]: terminal setup/teardown and event loop.

mod app;
mod events;
mod form;
mod runtime;
mod view;

pub use runtime::run;
