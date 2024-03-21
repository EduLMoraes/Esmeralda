use super::*;

#[allow(dead_code)]
pub fn get_grid_debtors() -> Grid {
    let grid_debtors = Grid::new();
    grid_debtors.set_halign(gtk::Align::Center);
    grid_debtors.set_column_spacing(10);
    grid_debtors.set_row_spacing(10);

    let debtors: Vec<Debtor> = Vec::new();

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
