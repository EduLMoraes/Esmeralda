use super::*;

#[allow(dead_code)]
pub fn get_grid_groups(stack: &Stack, stack_home: &Stack) -> Grid {
    let grid_groups = Grid::new();
    grid_groups.set_halign(gtk::Align::Center);
    grid_groups.set_column_spacing(10);
    grid_groups.set_row_spacing(10);

    let counts = get_counts_instance();

    let rnt = tokio::runtime::Runtime::new().unwrap();
    let natures = match rnt.block_on(control::get_groups()) {
        Ok(groups) => groups,
        Err(err) => {
            println!("{:?}", err);
            vec![
                String::from("Casa"),
                String::from("Transporte"),
                String::from("Alimentação"),
                String::from("Saúde"),
                String::from("Lazer"),
                String::from("Receita"),
                String::from("Outros"),
            ]
        }
    };

    for i in 0..natures.len() {
        grid_groups.attach(
            &new_group_info(
                &natures[i],
                &natures[i].to_lowercase(),
                &counts.filter_by_nature(&natures[i]),
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
