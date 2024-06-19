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

    // grid_groups.attach(
    //     &new_group_info(
    //         "Casa",
    //         "casa",
    //         &counts.search(String::from("Casa")),
    //         stack,
    //         stack_home,
    //     ),
    //     0,
    //     0,
    //     1,
    //     1,
    // );
    // grid_groups.attach(
    //     &new_group_info(
    //         "Transporte",
    //         "transporte",
    //         &counts.search(String::from("Transporte")),
    //         stack,
    //         stack_home,
    //     ),
    //     0,
    //     1,
    //     1,
    //     1,
    // );
    // grid_groups.attach(
    //     &new_group_info(
    //         "Alimentação",
    //         "alimentação",
    //         &counts.search(String::from("Alimentação")),
    //         stack,
    //         stack_home,
    //     ),
    //     0,
    //     2,
    //     1,
    //     1,
    // );
    // grid_groups.attach(
    //     &new_group_info(
    //         "Saúde",
    //         "saúde",
    //         &counts.search(String::from("Saúde")),
    //         stack,
    //         stack_home,
    //     ),
    //     1,
    //     0,
    //     1,
    //     1,
    // );
    // grid_groups.attach(
    //     &new_group_info(
    //         "Lazer",
    //         "lazer",
    //         &counts.search(String::from("Lazer")),
    //         stack,
    //         stack_home,
    //     ),
    //     1,
    //     1,
    //     1,
    //     1,
    // );
    // grid_groups.attach(
    //     &new_group_info(
    //         "Outros",
    //         "outros",
    //         &counts.search(String::from("Outros")),
    //         stack,
    //         stack_home,
    //     ),
    //     1,
    //     2,
    //     1,
    //     1,
    // );

    grid_groups
}
