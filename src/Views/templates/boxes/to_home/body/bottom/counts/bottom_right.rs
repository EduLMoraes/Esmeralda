use super::*;

pub fn right() -> Box {
    let box_right = Box::new(Orientation::Vertical, 20);

    box_right.add_css_class("box_right_bb");

    let history = Label::new(Some("Histórico"));
    history.set_margin_start(20);

    let order_by = Label::new(Some("Ordernar por: "));
    let drop_order = DropDown::from_strings(&[
        "Entrada ⤒⤓",
        "Data limite ⤒⤓",
        "Valor ⤒⤓",
        "Natureza ⤒⤓",
        "Status ⤒⤓",
        "Devedor ⤒⤓",
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

    let box_list_count = Box::new(Orientation::Vertical, 8);
    box_list_count.set_halign(gtk::Align::Center);

    let counts = unsafe { GLOBAL_COUNTS.get() };

    match counts {
        Some(counts) => {
            for count in &counts.list {
                box_list_count.append(&new_box_info(count));
            }
        }
        None => {}
    }

    scrolled.set_child(Some(&box_list_count));

    box_right.append(&box_head);
    box_right.append(&scrolled);
    box_right
}
