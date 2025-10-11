pub mod config;
pub mod tests;

mod user_controller;
pub use user_controller::*;

mod people_controller;
pub use people_controller::*;

mod data_controller;
pub use data_controller::*;

mod view_controller;
pub use view_controller::*;

mod file_controller;

pub use file_controller::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
