use super::*;
use crate::model::Debtor::Debtor;

pub fn new_debtor_info(debtor: &Debtor) -> Box {
    let box_debtors = Box::new(Orientation::Horizontal, 0);
    box_debtors.add_css_class("box_debtors");

    let grid: Grid = Grid::new();

    let name: Label = Label::new(Some(debtor.get_name()));

    let debt_value: Label = Label::new(Some(&format!("{:.2}", debtor.get_debt())));
    let paid_value: Label = Label::new(Some(&format!("{:.2}", debtor.get_value())));
    let total_value: Label = Label::new(Some(&format!(
        "{:.2}",
        debtor.get_value() + debtor.get_debt()
    )));

    let debt_label: Label = Label::new(Some("Devendo:"));
    let paid_label: Label = Label::new(Some("Pago:"));
    let total_label: Label = Label::new(Some("Pago:"));
    debt_label.add_css_class("title_i");
    paid_label.add_css_class("title_i");
    total_label.add_css_class("title_i");

    grid.attach(&name, 0, 0, 1, 1);
    grid.attach(&debt_label, 0, 1, 1, 1);
    grid.attach(&paid_label, 0, 2, 1, 1);
    grid.attach(&total_label, 0, 3, 1, 1);
    grid.attach(&debt_value, 1, 1, 1, 1);
    grid.attach(&paid_value, 1, 2, 1, 1);
    grid.attach(&total_value, 1, 3, 1, 1);

    box_debtors.append(&grid);

    box_debtors
}
