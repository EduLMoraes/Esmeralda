use super::*;
use crate::model::Info::Info;

#[allow(dead_code)]
pub fn get_grid_groups(stack: &Stack) -> Grid {
    let grid_groups = Grid::new();
    grid_groups.set_halign(gtk::Align::Center);
    grid_groups.set_column_spacing(10);
    grid_groups.set_row_spacing(10);

    let infos1: Vec<Info> = Vec::new();
    let infos2: Vec<Info> = Vec::new();
    let infos3: Vec<Info> = Vec::new();
    let infos4: Vec<Info> = Vec::new();
    let infos5: Vec<Info> = Vec::new();
    let infos6: Vec<Info> = Vec::new();

    grid_groups.attach(&new_group_info("Casa", "casa", &infos1, &stack), 0, 0, 1, 1);
    grid_groups.attach(
        &new_group_info("Transporte", "transporte", &infos2, &stack),
        0,
        1,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info("Alimentação", "alimentação", &infos3, &stack),
        0,
        2,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info("Saúde", "saúde", &infos4, &stack),
        1,
        0,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info("Lazer", "lazer", &infos5, &stack),
        1,
        1,
        1,
        1,
    );
    grid_groups.attach(
        &new_group_info("Outros", "outros", &infos6, &stack),
        1,
        2,
        1,
        1,
    );

    grid_groups
}
