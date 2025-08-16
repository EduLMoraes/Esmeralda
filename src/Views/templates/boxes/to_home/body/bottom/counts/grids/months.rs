use super::*;

#[allow(dead_code)]
pub fn get_grid_months(stack: &Stack, stack_home: &Stack) -> Grid {
    let grid_months = Grid::builder()
        .halign(gtk::Align::Fill)
        .column_homogeneous(true)
        .column_spacing(10)
        .row_spacing(10)
        .hexpand(true)
        .vexpand(true)
        .build();

    let counts = get_counts_instance();

    let mut months: Vec<(String, Vec<Count>)> = vec![
        (String::from("Janeiro"), Vec::new()),
        (String::from("Fevereiro"), Vec::new()),
        (String::from("Março"), Vec::new()),
        (String::from("Abril"), Vec::new()),
        (String::from("Maio"), Vec::new()),
        (String::from("Junho"), Vec::new()),
        (String::from("Julho"), Vec::new()),
        (String::from("Agosto"), Vec::new()),
        (String::from("Setembro"), Vec::new()),
        (String::from("Outubro"), Vec::new()),
        (String::from("Novembro"), Vec::new()),
        (String::from("Dezembro"), Vec::new()),
    ];

    months = counts.filter_by_month(months);

    for (i, month) in months.iter().enumerate() {
        grid_months.attach(
            &new_month_info(
                &month.0,
                &month.0.to_lowercase(),
                &month.1,
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
