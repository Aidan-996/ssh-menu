//! SSH connection + OpenSSH-config interop.
//!
//! Submodules:
//! - [`connect`]: build `ssh` argv, locate `ssh` binary, spawn the connection.
//! - [`import`]: parse `~/.ssh/config` and merge into ssh-menu's TOML store.

pub mod connect;
pub mod import;

pub use connect::{build_ssh_args, connect, now_rfc3339, time_ago};
pub use import::{merge_into, parse_ssh_config};
