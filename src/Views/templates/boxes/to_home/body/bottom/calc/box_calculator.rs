use super::*;
// mod normal;
// mod tax_box;
#[path = "./tax_box.rs"]
mod yield_prediction;

pub fn get_calculator_box() -> Box {
    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    let box_index = Box::new(Orientation::Vertical, 0);
    box_index.add_css_class("box_calculator_index");

    stack.add_titled(
        &yield_prediction::get_box(),
        Some("yield_prediction"),
        "Calculadora de Dividendo",
    );

    stack.set_visible_child_name("yield_prediciton");
    stack_switcher.set_stack(Some(&stack));

    box_index.append(&stack_switcher);
    box_index.append(&stack);
    box_index
}
