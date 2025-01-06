use super::*;

#[allow(dead_code)]
pub fn get_grid_values(count: f32, debt: f32, paid: f32, month: Vec<f32>) -> Grid {
    let grid = Grid::builder()
    .halign(gtk::Align::Center)
    .column_homogeneous(true)
    .row_spacing(10)
    .hexpand(true)
    .build();

    let box_count = Box::new(Orientation::Vertical, 10);

    let title = Label::new(Some("Total movimentado no ano"));
    let text = Label::new(Some(&format!("R$ {:.2}", count)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");

    box_count.append(&title);
    box_count.append(&text);

    let box_debt = Box::new(Orientation::Vertical, 10);
    box_debt.set_halign(gtk::Align::Center);
    box_debt.set_valign(gtk::Align::Center);

    let title = Label::new(Some("Total em dívidas abertas"));
    let text = Label::new(Some(&format!("R$ -{:.2}", debt)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");
    text.add_css_class("status_negative");

    box_debt.append(&title);
    box_debt.append(&text);

    let box_paid = Box::new(Orientation::Vertical, 10);
    box_paid.set_halign(gtk::Align::Center);
    box_paid.set_valign(gtk::Align::Center);

    let title = Label::new(Some("Saldo total"));
    let text = Label::new(Some(&format!("R$ {:.2}", paid)));
    title.add_css_class("name_i");
    text.add_css_class("value_total");
    if paid > 0.0 {
        text.add_css_class("status_positive");
    } else {
        text.add_css_class("status_negative");
    }

    box_paid.append(&title);
    box_paid.append(&text);

    let box_count_month = Box::new(Orientation::Vertical, 10);

    use chrono::Utc;
    let month_index = Utc::now();
    let months = [
        "Janeiro",
        "Fevereiro",
        "Março",
        "Abril",
        "Maio",
        "Junho",
        "Julho",
        "Agosto",
        "Setembro",
        "Outubro",
        "Novembro",
        "Dezembro",
    ];

    let title = Label::new(Some(&format!(
        "Perfomance em {}",
        months[month_index.month0() as usize]
    )));
    let text = Label::new(Some(&format!(
        "R$ {:.2}",
        month[month_index.month0() as usize]
    )));
    title.add_css_class("name_i");
    text.add_css_class("value_total");

    box_count_month.append(&title);
    box_count_month.append(&text);

    grid.attach(&box_count, 0, 0, 1, 1);
    grid.attach(&box_debt, 1, 0, 1, 1);
    grid.attach(&box_paid, 2, 0, 1, 1);

    grid.attach(&box_count_month, 1, 1, 1, 1);

    grid
}
