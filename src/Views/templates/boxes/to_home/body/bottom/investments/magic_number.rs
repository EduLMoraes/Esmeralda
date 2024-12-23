use super::*;
use crate::apis::get_quote;
use gtk::{Adjustment, CheckButton, SpinButton};

/// this calcule a preview of "magic number"
/// magic number is quantity of action to value of yield = value per action
pub fn get_box() -> Box {
    let box_index = Box::new(Orientation::Horizontal, 2);
    box_index.add_css_class("box_calculator");

    let name_label = Label::new(Some("Cota:"));
    let vpa_label = Label::new(Some("Valor p/ Cota(R$):"));
    let actions_label = Label::new(Some("Cotas(Un):"));
    let yield_tax_label = Label::new(Some("Taxa de dividendo(%):"));
    let magic_number_label = Label::new(Some("Número Mágico:"));
    let check_month_label = Label::new(Some("Mensal:"));
    let check_year_label = Label::new(Some("Anual:"));
    let yield_label = Label::new(Some("Dividendos Recebidos:"));
    let total_invest_label = Label::new(Some("Total investido:"));

    let name_input = Entry::new();
    let vpa_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 0.01, 0.1, 0.0)),
        1.0,
        2,
    );
    let actions_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        0,
    );

    let yield_tax_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 0.1, 0.0)),
        1.0,
        2,
    );
    let magic_number_input = SpinButton::new(
        Some(&Adjustment::new(0.0, 0.0, 9999999.0, 1.0, 1.0, 0.0)),
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

    name_input.add_css_class("input_calculator");
    yield_input.add_css_class("input_calculator");
    vpa_input.add_css_class("input_calculator");
    actions_input.add_css_class("input_calculator");
    yield_tax_input.add_css_class("input_calculator");
    magic_number_input.add_css_class("input_calculator");
    total_invest_input.add_css_class("input_calculator");

    name_input.set_halign(gtk::Align::Start);
    vpa_input.set_halign(gtk::Align::Start);
    actions_input.set_halign(gtk::Align::Start);
    yield_tax_input.set_halign(gtk::Align::Start);
    magic_number_input.set_halign(gtk::Align::Start);
    yield_input.set_halign(gtk::Align::Start);
    total_invest_input.set_halign(gtk::Align::Start);

    name_label.set_halign(gtk::Align::Start);
    vpa_label.set_halign(gtk::Align::Start);
    actions_label.set_halign(gtk::Align::Start);
    yield_tax_label.set_halign(gtk::Align::Start);
    magic_number_label.set_halign(gtk::Align::Start);
    yield_label.set_halign(gtk::Align::Start);
    total_invest_label.set_halign(gtk::Align::Start);

    name_input.connect_activate(clone!(
        #[weak]
        name_input,
        #[weak]
        vpa_input,
        #[weak]
        yield_tax_input,
        move |_| {
            match get_quote(format!("{}.sa", &name_input.text()).to_uppercase().trim()) {
                Ok((quote, Some(dividend))) => {
                    vpa_input.set_value(quote.close);
                    yield_tax_input.set_value((dividend.amount * 12.0 * 100.0) / quote.close);
                }
                Ok((quote, None)) => vpa_input.set_value(quote.close),
                Err(_yerror) => {}
            };
        }
    ));

    auto_complete(
        &vpa_input,
        &actions_input,
        &vpa_input,
        &total_invest_input,
        &yield_input,
        &magic_number_input,
        &yield_tax_input,
        &check_month_input,
    );
    auto_complete(
        &yield_input,
        &actions_input,
        &vpa_input,
        &total_invest_input,
        &yield_input,
        &magic_number_input,
        &yield_tax_input,
        &check_month_input,
    );
    auto_complete(
        &yield_tax_input,
        &actions_input,
        &vpa_input,
        &total_invest_input,
        &yield_input,
        &magic_number_input,
        &yield_tax_input,
        &check_month_input,
    );
    auto_complete(
        &actions_input,
        &actions_input,
        &vpa_input,
        &total_invest_input,
        &yield_input,
        &magic_number_input,
        &yield_tax_input,
        &check_month_input,
    );

    let box_name = Box::new(Orientation::Vertical, 1);
    let box_vpa = Box::new(Orientation::Vertical, 1);
    let box_actions = Box::new(Orientation::Vertical, 1);
    let box_yield_tax = Box::new(Orientation::Vertical, 1);
    let box_magic_number = Box::new(Orientation::Vertical, 1);
    let box_check_month = Box::new(Orientation::Horizontal, 1);
    let box_check_year = Box::new(Orientation::Horizontal, 1);
    let box_yield = Box::new(Orientation::Vertical, 1);
    let box_total_invest = Box::new(Orientation::Vertical, 1);
    let box_time = Box::new(Orientation::Vertical, 1);

    box_name.set_halign(gtk::Align::Start);
    box_vpa.set_halign(gtk::Align::Start);
    box_actions.set_halign(gtk::Align::Start);
    box_yield_tax.set_halign(gtk::Align::Start);
    box_magic_number.set_halign(gtk::Align::Start);
    box_yield.set_halign(gtk::Align::Start);
    box_total_invest.set_halign(gtk::Align::Start);
    box_time.set_halign(gtk::Align::Start);

    box_name.append(&name_label);
    box_name.append(&name_input);
    box_vpa.append(&vpa_label);
    box_vpa.append(&vpa_input);
    box_actions.append(&actions_label);
    box_actions.append(&actions_input);
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
    grid.attach(&box_yield_tax, 3, 0, 1, 1);
    grid.attach(&box_magic_number, 4, 0, 1, 1);
    grid.attach(&box_yield, 0, 1, 1, 1);
    grid.attach(&box_total_invest, 1, 1, 1, 1);
    grid.attach(&box_check_month, 2, 1, 1, 1);
    grid.attach(&box_check_year, 3, 1, 1, 1);

    box_index.append(&grid);
    box_index
}

fn auto_complete(
    input: &SpinButton,
    actions: &SpinButton,
    value: &SpinButton,
    total: &SpinButton,
    yields: &SpinButton,
    magic: &SpinButton,
    yields_tax: &SpinButton,
    is_month: &CheckButton,
) {
    input.connect_value_changed(clone!(
        #[weak]
        actions,
        #[weak]
        total,
        #[weak]
        value,
        #[weak]
        magic,
        #[weak]
        yields,
        #[weak]
        yields_tax,
        #[weak]
        is_month,
        move |_| {
            let yield_year = &value.value() * (&yields_tax.value() / 100.0);
            let yield_month = yield_year / 12.0;

            if yields_tax.value() > 0.0 && value.value() > 0.0 {
                magic.set_value(&value.value() / yield_month + 1.0);
            }

            if is_month.is_active() {
                yields.set_value(actions.value() * yield_month);
            } else {
                yields.set_value(actions.value() * yield_year);
            }

            total.set_value(&actions.value() * &value.value());
        }
    ));
}
