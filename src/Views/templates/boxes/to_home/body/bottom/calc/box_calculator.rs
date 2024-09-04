use super::*;
// mod normal;
// mod tax_box;
// mod yield_prediction;

pub fn get_calculator_box() -> Box {
    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    let box_index = Box::new(Orientation::Vertical, 0);
    box_index.add_css_class("box_calculator_index");

    let label = Label::new(Some("Olá"));
    box_index.append(&label);
    box_index
}