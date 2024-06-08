use super::*;

pub fn update_list(list: Option<&ListCount>, stack: std::option::Option<&Stack>) {
    let list_box = get_list_box();
    list_box.remove_all();

    let counts = match list {
        Some(counts) => counts,
        None => unsafe { GLOBAL_COUNTS.borrow() },
    };

    use crate::utils::export::svg;
    svg::to_svg(
        var("YEAR_SELECTED").unwrap().parse::<i16>().unwrap(),
        counts.get_data_months(),
        counts.filter_debtors(),
    );

    for count in &counts.list {
        list_box.append(&new_box_info(count));
    }

    match stack {
        Some(stack) => {
            match stack.child_by_name("home") {
                Some(tmp) => {
                    stack.remove(&tmp);
                    stack.add_titled(&get_home_box(&stack), Some("home"), "home");
                }
                None => {}
            }

            match stack.child_by_name("payment") {
                Some(tmp) => {
                    stack.remove(&tmp);
                    stack.add_titled(&get_pay_box(&stack), Some("payment"), "payment");
                }
                None => {}
            }

            match stack.child_by_name("Contas") {
                Some(tmp) => {
                    stack.remove(&tmp);
                    stack.add_titled(&box_count(), Some("Contas"), "Contas");
                }
                None => {}
            }

            match stack.child_by_name("Graficos") {
                Some(tmp) => {
                    stack.remove(&tmp);
                    stack.add_titled(&box_graph(), Some("Graficos"), "Graficos");
                }
                None => {}
            }
        }
        None => {}
    }
}
