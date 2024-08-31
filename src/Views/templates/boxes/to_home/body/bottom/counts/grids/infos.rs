use super::*;
use crate::model::Count::Count;

#[allow(dead_code)]
pub fn get_grid_infos(stack: &Stack, stack_home: &Stack, infos: &Vec<Count>) -> Grid {
    let grid_infos = Grid::new();
    grid_infos.set_halign(gtk::Align::Center);
    grid_infos.set_column_spacing(10);
    grid_infos.set_row_spacing(10);

    let mut x = 0;
    let mut y = 0;

    for info in infos {
        let group = box_info(info, stack_home);
        grid_infos.attach(&group, x, y, 1, 1);

        if x < 1 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }

    if let Some(child) = stack.child_by_name("details") {
        stack.remove(&child);
    }

    stack.add_titled(&grid_infos, Some("details"), &infos[0].nature);
    grid_infos
}
