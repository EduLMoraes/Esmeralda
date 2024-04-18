use crate::env::var;
use crate::prelude::control;
pub use gtk::prelude::*;
pub use gtk::{
    glib::clone, Application, Box, Button, Entry, Label, LinkButton, Orientation, Stack,
    StackSwitcher,
};

mod app;
pub use app::esmeralda;
