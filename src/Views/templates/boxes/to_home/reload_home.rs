use super::*;

pub fn reload_home(list: Option<&ListCount>, stack: std::option::Option<&Stack>) {
    let list_box = get_list_box();

    while list_box.first_child() != None {
        let first_child = list_box.first_child().unwrap();
        list_box.remove(&first_child);
    }

    let binding = get_counts_instance().clone();
    let counts = match list {
        Some(counts) => counts,
        None => &binding,
    };

    for count in &counts.list {
        list_box.append(&new_box_info(count));
    }

    use crate::utils::export::svg;
    svg::to_svg(counts.get_data_months(), counts.filter_debtors());

    if let Some(stack) = stack {
        if let Some(tmp) = stack.child_by_name("home") {
            stack.remove(&tmp);
            stack.add_titled(&get_home_box(stack), Some("home"), "home");
        }

        if let Some(tmp) = stack.child_by_name("payment") {
            stack.remove(&tmp);
            stack.add_titled(&get_pay_box(stack), Some("payment"), "payment");
        }

        if let Some(tmp) = stack.child_by_name("Contas") {
            stack.remove(&tmp);
            stack.add_titled(&box_count(), Some("Contas"), "Contas");
        }

        if let Some(tmp) = stack.child_by_name("Graficos") {
            stack.remove(&tmp);
            stack.add_titled(&box_graph(), Some("Graficos"), "Graficos");
        }
    }

    let _ = std::mem::drop(binding);
}
