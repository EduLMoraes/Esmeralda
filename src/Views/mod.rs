use crate::env::var;
use crate::prelude::control;
pub use gtk::prelude::*;
pub use gtk::{
    gdk::Display, glib::clone, Application, ApplicationWindow, Box, Button, Entry, Label,
    LinkButton, Orientation, Stack, StackSwitcher,
};

mod app;
pub use app::esmeralda;
