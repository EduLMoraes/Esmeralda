use gtk::{builders::CheckButtonBuilder, Adjustment, CheckButton, SpinButton};

use super::*;

/// this calcule a preview of "magic number"
/// magic number is quantity of action to value of yield = value per action
pub fn get_box() -> Box {
    let box_index = Box::new(Orientation::Horizontal, 2);
    box_index.add_css_class("box_calculator");

    let name_label = Label::new(Some("Ação:"));
    /// name of action
    let vpa_label = Label::new(Some("Valor p/ Ação:"));
    /// value per action
    let actions_label = Label::new(Some("Nª de Ações:"));
    let ymn_label = Label::new(Some("Anos até alcançar o número mágico:"));
    /// years to magic number
    let yield_tax_label = Label::new(Some("Taxa de dividendo:"));
    let magic_number_label = Label::new(Some("Número Mágico:"));
    let check_month_label = Label::new(Some("Mensal:"));
    let check_year_label = Label::new(Some("Anual:"));
    let yield_label = Label::new(Some("Dividendos:"));
    let total_invest_label = Label::new(Some("Total investido:"));

    let name_input = Entry::new();
    /// name of action
    let vpa_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );
    /// value per action
    let actions_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    let ymn_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    /// years to magic number
    let yield_tax_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        2,
    );
    let magic_number_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );
    let check_month_input = CheckButton::new();
    let check_year_input = CheckButton::new();
    let yield_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );
    let total_invest_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );

    check_year_input.set_group(Some(&check_month_input));
    check_year_input.set_active(true);
    magic_number_input.set_editable(false);
    ymn_input.set_editable(false);

    name_input.add_css_class("input_calculator");
    yield_input.add_css_class("input_calculator");
    vpa_input.add_css_class("input_calculator");
    actions_input.add_css_class("input_calculator");
    ymn_input.add_css_class("input_calculator");
    yield_tax_input.add_css_class("input_calculator");
    magic_number_input.add_css_class("input_calculator");
    total_invest_input.add_css_class("input_calculator");

    name_input.set_halign(gtk::Align::Start);
    vpa_input.set_halign(gtk::Align::Start);
    actions_input.set_halign(gtk::Align::Start);
    ymn_input.set_halign(gtk::Align::Start);
    yield_tax_input.set_halign(gtk::Align::Start);
    magic_number_input.set_halign(gtk::Align::Start);
    yield_input.set_halign(gtk::Align::Start);
    total_invest_input.set_halign(gtk::Align::Start);

    name_label.set_halign(gtk::Align::Start);
    vpa_label.set_halign(gtk::Align::Start);
    actions_label.set_halign(gtk::Align::Start);
    ymn_label.set_halign(gtk::Align::Start);
    yield_tax_label.set_halign(gtk::Align::Start);
    magic_number_label.set_halign(gtk::Align::Start);
    yield_label.set_halign(gtk::Align::Start);
    total_invest_label.set_halign(gtk::Align::Start);

    vpa_input.connect_value_changed(clone!(
        #[weak]
        actions_input,
        #[weak]
        total_invest_input,
        #[weak]
        vpa_input,
        #[weak]
        yield_input,
        move |_| {
            total_invest_input.set_value(&actions_input.value() * &vpa_input.value() - yield_input.value());
        }
    ));

    actions_input.connect_value_changed(clone!(
        #[weak]
        actions_input,
        #[weak]
        total_invest_input,
        #[weak]
        vpa_input,
        #[weak]
        yield_input,
        move |_| {
            total_invest_input.set_value(&actions_input.value() * &vpa_input.value() - yield_input.value());
        }
    ));

    yield_tax_input.connect_value_changed(clone!(
        #[weak]
        actions_input,
        #[weak]
        total_invest_input,
        #[weak]
        vpa_input,
        #[weak]
        yield_input,
        move |_| {
            total_invest_input.set_value(&actions_input.value() * &vpa_input.value() - yield_input.value());
        }
    ));

    let box_name = Box::new(Orientation::Vertical, 1);
    let box_vpa = Box::new(Orientation::Vertical, 1);
    let box_actions = Box::new(Orientation::Vertical, 1);
    let box_ymn = Box::new(Orientation::Vertical, 1);
    let box_yield_tax = Box::new(Orientation::Vertical, 1);
    let box_magic_number = Box::new(Orientation::Vertical, 1);
    let box_check_month = Box::new(Orientation::Horizontal, 1);
    let box_check_year = Box::new(Orientation::Horizontal, 1);
    let box_yield = Box::new(Orientation::Vertical, 1);
    let box_total_invest = Box::new(Orientation::Vertical, 1);

    box_name.set_halign(gtk::Align::Start);
    box_vpa.set_halign(gtk::Align::Start);
    box_actions.set_halign(gtk::Align::Start);
    box_ymn.set_halign(gtk::Align::Start);
    box_yield_tax.set_halign(gtk::Align::Start);
    box_magic_number.set_halign(gtk::Align::Start);
    box_yield.set_halign(gtk::Align::Start);
    box_total_invest.set_halign(gtk::Align::Start);

    box_name.append(&name_label);
    box_name.append(&name_input);
    box_vpa.append(&vpa_label);
    box_vpa.append(&vpa_input);
    box_actions.append(&actions_label);
    box_actions.append(&actions_input);
    box_ymn.append(&ymn_label);
    box_ymn.append(&ymn_input);
    box_yield_tax.append(&yield_tax_label);
    box_yield_tax.append(&yield_tax_input);
    box_magic_number.append(&magic_number_label);
    box_magic_number.append(&magic_number_input);
    box_check_month.append(&check_month_label);
    box_check_month.append(&check_month_input);
    box_check_year.append(&check_year_label);
    box_check_year.append(&check_year_input);
    box_yield.append(&yield_label);
    box_yield.append(&yield_input);
    box_total_invest.append(&total_invest_label);
    box_total_invest.append(&total_invest_input);

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    grid.attach(&box_name, 0, 0, 1, 1);
    grid.attach(&box_actions, 1, 0, 1, 1);
    grid.attach(&box_vpa, 2, 0, 1, 1);
    grid.attach(&box_yield_tax, 0, 1, 1, 1);
    grid.attach(&box_yield, 1, 1, 1, 1);
    grid.attach(&box_total_invest, 2, 1, 1, 1);
    grid.attach(&box_check_month, 0, 2, 1, 1);
    grid.attach(&box_check_year, 1, 2, 1, 1);
    grid.attach(&box_magic_number, 3, 0, 1, 1);
    grid.attach(&box_ymn, 3, 1, 1, 1);

    box_index.append(&grid);
    box_index
}
