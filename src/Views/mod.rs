use crate::env::var;
use crate::prelude::control;
use crate::prelude::log;
use crate::prelude::model::errors::*;
pub use gtk::prelude::*;
pub use gtk::{
    gdk::Display, glib::clone, Application, ApplicationWindow, Box, Button, Entry, Label,
    LinkButton, Orientation, Stack, StackSwitcher,
};

mod app;
pub use app::esmeralda;
