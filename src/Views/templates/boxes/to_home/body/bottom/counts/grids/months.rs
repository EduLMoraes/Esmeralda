use super::*;

#[allow(dead_code)]
pub fn get_grid_months(stack: &Stack, stack_home: &Stack) -> Grid {
    let grid_months = Grid::new();
    grid_months.set_halign(gtk::Align::Center);
    grid_months.set_column_spacing(10);
    grid_months.set_row_spacing(10);

    let counts = get_counts_instance();

    let months = vec![
        String::from("Janeiro"),
        String::from("Fevereiro"),
        String::from("Março"),
        String::from("Abril"),
        String::from("Maio"),
        String::from("Junho"),
        String::from("Julho"),
        String::from("Agosto"),
        String::from("Setembro"),
        String::from("Outubro"),
        String::from("Novembro"),
        String::from("Dezembro"),
    ];

    for i in 0..months.len() {
        grid_months.attach(
            &new_month_info(
                &months[i],
                &months[i].to_lowercase(),
                &counts.filter_by_month(&(i as u32)),
                stack,
                stack_home,
            ),
            (i % 2) as i32,
            (i - (i % 2)) as i32,
            1,
            1,
        );
    }

    grid_months
}
