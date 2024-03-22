use super::*;

#[allow(dead_code)]
pub fn get_grid_groups(stack: &Stack) -> Grid {
    let grid_groups = Grid::new();
    grid_groups.set_halign(gtk::Align::Center);
    grid_groups.set_column_spacing(10);
    grid_groups.set_row_spacing(10);

    let counts = unsafe {
        match GLOBAL_COUNTS.get() {
            Some(counts) => counts.to_owned(),
            None => ListInfo::new(),
        }
    };

    grid_groups.attach(
        &new_group_info("Casa", "casa", &counts.search(String::from("Casa")), &stack),
        0,
        0,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info(
            "Transporte",
            "transporte",
            &counts.search(String::from("Transporte")),
            &stack,
        ),
        0,
        1,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info(
            "Alimentação",
            "alimentação",
            &counts.search(String::from("Alimentação")),
            &stack,
        ),
        0,
        2,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info(
            "Saúde",
            "saúde",
            &counts.search(String::from("Saúde")),
            &stack,
        ),
        1,
        0,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info(
            "Lazer",
            "lazer",
            &counts.search(String::from("Lazer")),
            &stack,
        ),
        1,
        1,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info(
            "Outros",
            "outros",
            &counts.search(String::from("Outros")),
            &stack,
        ),
        1,
        2,
        1,
        1,
    );

    grid_groups
}
