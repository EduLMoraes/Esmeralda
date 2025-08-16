use super::*;

pub fn right() -> Box {
    let box_right = Box::new(Orientation::Vertical, 20);
    box_right.set_hexpand(true);
    box_right.set_vexpand(true);
    box_right.set_halign(gtk::Align::Fill);
    box_right.add_css_class("box_right_bb");

    let history = Label::new(Some("Histórico"));

    let order_by = Label::new(Some("Ordernar por: "));
    let drop_order = DropDown::from_strings(&[
        "⤓ Entrada",
        "⤒ Entrada",
        "⤓ Data limite",
        "⤒ Data limite",
        "⤓ Valor",
        "⤒ Valor",
        "⤓ Natureza",
        "⤒ Natureza",
        "⤓ Status",
        "⤒ Status",
        "⤓ Devedor",
        "⤒ Devedor",
    ]);
    drop_order.set_css_classes(&["dropdown_order_by"]);

    let box_order = Box::new(Orientation::Horizontal, 0);
    box_order.append(&order_by);
    box_order.append(&drop_order);

    let box_head = Box::new(Orientation::Horizontal, 100);
    box_head.append(&history);
    box_head.append(&box_order);

    box_head.add_css_class("box_head_bbr");
    box_head.set_hexpand(true);
    box_head.set_halign(gtk::Align::Center);

    let scrolled = ScrolledWindow::new();
    scrolled.add_css_class("list_info_history");
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let box_list_count = get_list_box();
    box_list_count.add_css_class("list_info_history");
    box_list_count.set_vexpand(true);
    box_list_count.set_hexpand(true);

    let counts = get_counts_instance();

    drop_order.connect_selected_item_notify(move |drop_order| {
        let mut counts = get_counts_instance().clone();

        match drop_order.selected() {
            0 => counts.order_by_date(true, true),
            1 => counts.order_by_date(true, false),
            2 => counts.order_by_date(false, true),
            3 => counts.order_by_date(false, false),
            4 => counts.order_by_value(true),
            5 => counts.order_by_value(false),
            6 => counts.order_alphabetical("nature", true),
            7 => counts.order_alphabetical("nature", false),
            8 => counts.order_by_status(true),
            9 => counts.order_by_status(false),
            10 => counts.order_alphabetical("name", true),
            11 => counts.order_alphabetical("name", false),
            _ => {}
        }

        reload_home(Some(&counts), None);
        std::mem::drop(counts);
    });

    for count in &counts.list {
        box_list_count.append(&new_box_info(count));
    }

    std::mem::drop(counts);

    scrolled.set_child(Some(box_list_count));

    box_right.append(&box_head);
    box_right.append(&scrolled);
    box_right
}
