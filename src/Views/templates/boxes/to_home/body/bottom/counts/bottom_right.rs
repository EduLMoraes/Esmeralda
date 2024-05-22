use super::*;

pub fn right() -> Box {
    let box_right = Box::new(Orientation::Vertical, 20);
    box_right.set_hexpand(true);
    box_right.set_vexpand(true);
    box_right.add_css_class("box_right_bb");

    let history = Label::new(Some("Histórico"));
    history.set_margin_start(20);

    let order_by = Label::new(Some("Ordernar por: "));
    let drop_order = DropDown::from_strings(&[
        "Entrada",
        "Data limite",
        "Valor",
        "Natureza",
        "Status",
        "Devedor",
    ]);

    let box_order = Box::new(Orientation::Horizontal, 0);
    box_order.append(&order_by);
    box_order.append(&drop_order);

    let box_head = Box::new(Orientation::Horizontal, 100);
    box_head.append(&history);
    box_head.append(&box_order);
    box_head.add_css_class("box_head_bbr");

    let scrolled = ScrolledWindow::new();
    scrolled.add_css_class("list_info_history");
    scrolled.set_hexpand(true);

    let box_list_count = get_list_box();
    box_list_count.add_css_class("list_info_history");
    box_list_count.set_vexpand(true);

    let counts = unsafe { GLOBAL_COUNTS.borrow() };
    drop_order.connect_selected_item_notify(move |drop_order| {
        let counts = unsafe { GLOBAL_COUNTS.borrow_mut() };

        match drop_order.selected() {
            0 => counts.order_by_date(true, true),
            1 => counts.order_by_date(false, false),
            2 => counts.order_by_value(true),
            3 => counts.order_alphabetical("nature", true),
            4 => counts.order_by_status(true),
            5 => counts.order_alphabetical("name", true),
            _ => {}
        }

        update_list(counts);
    });

    for count in &counts.list {
        box_list_count.append(&new_box_info(count));
    }

    scrolled.set_child(Some(box_list_count));

    box_right.append(&box_head);
    box_right.append(&scrolled);
    box_right
}
