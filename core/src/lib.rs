#![feature(map_many_mut)]

pub mod app;
pub mod scheduler;
pub mod storage;

pub use app::{App, Config};
pub use scheduler::{schedule, system, Plugin, PluginBuilder, Scheduler};
pub use storage::Storage;
