// use self::templates::lost_pass_screen;

use super::*;
use gtk::Grid;
use std::borrow::{Borrow, BorrowMut};

#[path = "./templates/mod.rs"]
mod templates;
use templates::{login_screen, rgter_screen};

// #[path = "./alerts/mod.rs"]
// mod alerts;
// use alerts::*;

pub fn esmeralda(app: &Application) {
    let window = control::config::app::get_config(app);

    let stack = Stack::new();
    stack.set_css_classes(&["login_window", "window"]);

    stack.set_transition_type(gtk::StackTransitionType::Crossfade);
    stack.set_transition_duration(0);

    let login_screen = login_screen(&stack);
    let register_screen = rgter_screen(&stack);
    // let lost_pass_screen = lost_pass_screen(&stack);

    stack.add_titled(&login_screen, Some("login"), "Login");
    stack.add_titled(&register_screen, Some("register"), "Register");
    // stack.add_titled(&lost_pass_screen, Some("rem_pass"), "Rem_pass");

    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(&stack));

    stack.set_visible_child_name("login");

    let grid = Grid::new();

    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.attach(&stack, 0, 0, 1, 1);

    grid.set_row_spacing(0);
    grid.set_column_spacing(0);

    window.set_child(Some(&grid));
    window.present();
}
