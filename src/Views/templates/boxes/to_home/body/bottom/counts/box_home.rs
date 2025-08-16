use super::*;

use grids::*;

pub fn get_home_box(stack: &Stack) -> Box {
    let box_home = Box::new(Orientation::Vertical, 10);
    box_home.add_css_class("box_left_bb");
    box_home.set_hexpand(true);
    box_home.set_vexpand(true);
    box_home.set_valign(gtk::Align::Fill);
    box_home.set_halign(gtk::Align::Fill);

    let box_button_lb = Box::new(Orientation::Horizontal, 10);
    box_button_lb.set_halign(gtk::Align::Center);

    let button_add = Button::with_label("Adicionar");
    let button_payment = Button::with_label("Pagar");

    button_add.add_css_class("button_add");
    button_payment.add_css_class("button_payment");

    button_add.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            stack.set_visible_child_name("addition");
        }
    ));

    button_payment.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            stack.set_visible_child_name("payment");
        }
    ));

    box_button_lb.append(&button_add);
    box_button_lb.append(&button_payment);

    let stack_infos = Stack::new();
    let stack_switcher = StackSwitcher::new();
    stack_switcher.add_css_class("stack_switcher");

    stack_infos.add_titled(
        &get_grid_groups(&stack_infos, stack),
        Some("groups"),
        "Contas",
    );
    stack_infos.add_titled(&get_grid_debtors(), Some("debtors"), "Devedores");
    stack_infos.add_titled(
        &get_grid_months(&stack_infos, stack),
        Some("months"),
        "Meses",
    );
    stack_infos.set_visible_child_name("groups");

    stack_switcher.set_stack(Some(&stack_infos));

    let box_head_stack = Box::new(Orientation::Horizontal, 5);
    box_head_stack.append(&stack_switcher);

    let box_stack = Box::new(Orientation::Vertical, 5);
    box_stack.append(&box_head_stack);
    box_stack.append(&stack_infos);

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&box_stack));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    let mut binding = get_counts_instance();
    let counts = binding.borrow_mut();

    box_home.append(&get_grid_values(
        counts.get_total(),
        counts.get_total_debt(),
        counts.get_total_perfomance(),
        counts.get_perfomance_months(),
    ));
    box_home.append(&box_button_lb);
    box_home.append(&scrolled);

    box_home
}
