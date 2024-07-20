use super::*;

pub fn left() -> Box {
    let box_left = Box::new(Orientation::Vertical, 10);
    box_left.add_css_class("box_left_bb");
    box_left.set_hexpand(true);
    box_left.set_vexpand(true);

    let stack_left = Stack::new();
    stack_left.add_titled(&get_home_box(&stack_left), Some("home"), "home");
    stack_left.add_titled(&get_add_box(&stack_left), Some("addition"), "addition");
    stack_left.add_titled(&get_pay_box(&stack_left), Some("payment"), "payment");
    stack_left.set_vexpand(true);
    stack_left.set_hexpand(true);

    box_left.append(&stack_left);
    box_left
}
