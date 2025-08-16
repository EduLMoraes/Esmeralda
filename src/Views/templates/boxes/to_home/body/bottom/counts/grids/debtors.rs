use super::*;

#[allow(dead_code)]
pub fn get_grid_debtors() -> Grid {
    let grid_debtors = Grid::builder()
        .halign(gtk::Align::Fill)
        .column_homogeneous(true)
        .column_spacing(10)
        .hexpand(true)
        .vexpand(true)
        .build();

    let binding = get_counts_instance();
    let counts = binding.borrow();
    let debtors: Vec<Debtor> = counts.filter_debtors();

    let mut x = 0;
    let mut y = 0;

    for debtor in &debtors {
        grid_debtors.attach(&new_debtor_info(debtor), x, y, 1, 1);

        if x < 1 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }

    grid_debtors
}
