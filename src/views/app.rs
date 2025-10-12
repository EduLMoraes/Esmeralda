use crate::{
    controller::config::app::get_config,
    views::{login::ViewLogin, register::ViewRegister},
};
use gtk::{prelude::*, Application, Grid, Stack, StackSwitcher};
use std::cell::RefCell;

pub fn esmeralda(app: &Application) {
    let window = get_config(app);

    let stack = RefCell::new(Stack::new());
    stack.borrow().set_css_classes(&["login_window", "window"]);

    stack
        .borrow()
        .set_transition_type(gtk::StackTransitionType::Crossfade);
    stack.borrow().set_transition_duration(0);

    let login_screen = ViewLogin::new(stack.clone());
    let register_screen = ViewRegister::new(stack.clone());
    // let lost_pass_screen = lost_pass_screen(&stack);

    stack
        .borrow()
        .add_titled(&login_screen.login_screen(), Some("login"), "Login");
    stack.borrow().add_titled(
        &register_screen.rgter_screen(),
        Some("register"),
        "Register",
    );
    // stack.add_titled(&lost_pass_screen, Some("rem_pass"), "Rem_pass");

    let switcher = StackSwitcher::new();
    switcher.set_stack(Some(&stack.borrow()));

    stack.borrow().set_visible_child_name("login");

    let grid = Grid::new();

    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    grid.attach(&stack.take(), 0, 0, 1, 1);

    grid.set_row_spacing(0);
    grid.set_column_spacing(0);

    window.set_child(Some(&grid));
    window.present();
}
