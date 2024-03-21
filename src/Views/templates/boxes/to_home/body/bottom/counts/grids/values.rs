use super::*;

#[allow(dead_code)]
pub fn get_grid_values(count: f32, debt: f32, paid: f32) -> Grid {
    let grid = Grid::new();
    grid.set_column_homogeneous(true);

    let box_count = Box::new(Orientation::Vertical, 10);

    let title = Label::new(Some("Conta"));
    let text = Label::new(Some(&format!("R${:.2}", count)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");

    box_count.append(&title);
    box_count.append(&text);

    let box_debt = Box::new(Orientation::Vertical, 10);
    box_debt.set_halign(gtk::Align::Start);
    box_debt.set_valign(gtk::Align::Start);

    let title = Label::new(Some("Devendo"));
    let text = Label::new(Some(&format!("-R${:.2}", debt)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");
    text.add_css_class("status_negative");

    box_debt.append(&title);
    box_debt.append(&text);

    let box_paid = Box::new(Orientation::Vertical, 10);
    box_paid.set_halign(gtk::Align::Start);
    box_paid.set_valign(gtk::Align::Start);

    let title = Label::new(Some("Total pago"));
    let text = Label::new(Some(&format!("+R${:.2}", paid)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");
    text.add_css_class("status_positive");

    box_paid.append(&title);
    box_paid.append(&text);

    grid.attach(&box_count, 0, 0, 1, 1);
    grid.attach(&box_debt, 1, 0, 1, 1);
    grid.attach(&box_paid, 2, 0, 1, 1);

    grid
}
