use crate::prelude::apis::Navigator;
use crate::prelude::apis::Route;
use crate::prelude::control;
use crate::prelude::dioxus::prelude::*;
use crate::prelude::log;
use crate::prelude::model::errors::*;

/// Screens
mod home;
mod login;
mod register;
mod restore;

pub use home::*;
pub use login::*;
pub use register::*;
pub use restore::*;

pub mod scripts;
pub mod styles;
pub mod templates;
pub mod tests;
