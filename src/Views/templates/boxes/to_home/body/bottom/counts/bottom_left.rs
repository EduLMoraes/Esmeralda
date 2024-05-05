use super::*;

pub fn left() -> Box {
    let box_left = Box::new(Orientation::Vertical, 10);
    box_left.set_halign(gtk::Align::Center);
    box_left.set_valign(gtk::Align::Center);
    box_left.add_css_class("box_left_bb");

    let stack_left = Stack::new();
    stack_left.add_titled(&get_home_box(&stack_left), Some("home"), "home");
    stack_left.add_titled(&get_add_box(&stack_left), Some("addition"), "addition");
    stack_left.add_titled(&get_pay_box(&stack_left), Some("payment"), "payment");

    box_left.append(&stack_left);
    box_left
}
