use super::*;

#[allow(dead_code)]
pub fn get_grid_groups(stack: &Stack, stack_home: &Stack) -> Grid {
    let grid_groups = Grid::new();
    grid_groups.set_halign(gtk::Align::Center);
    grid_groups.set_column_spacing(10);
    grid_groups.set_row_spacing(10);

    let counts = get_counts_instance();

    let natures = vec![
        "Casa",
        "Transporte",
        "Alimentação",
        "Saúde",
        "Lazer",
        "Receita",
        "Outros",
    ];

    for i in 0..natures.len() {
        grid_groups.attach(
            &new_group_info(
                &natures[i],
                &natures[i].to_lowercase(),
                &counts.search(String::from(natures[i])),
                stack,
                stack_home,
            ),
            (i % 2) as i32,
            (i - (i % 2)) as i32,
            1,
            1,
        );
    }

    grid_groups
}
