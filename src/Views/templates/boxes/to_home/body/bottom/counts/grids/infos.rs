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
        let group = box_info(info, Some(stack_home));
        grid_infos.attach(&group, x, y as i32, 1, 1);

        if x < 1 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }

    if stack.child_by_name("Casa").as_ref().is_some() {
        let child = stack.child_by_name("Casa").unwrap();
        stack.remove(&child);
    } else if stack.child_by_name("Transporte").as_ref().is_some() {
        let child = stack.child_by_name("Transporte").unwrap();
        stack.remove(&child);
    } else if stack.child_by_name("Lazer").as_ref().is_some() {
        let child = stack.child_by_name("Lazer").unwrap();
        stack.remove(&child);
    } else if stack.child_by_name("Outros").as_ref().is_some() {
        let child = stack.child_by_name("Outros").unwrap();
        stack.remove(&child);
    } else if stack.child_by_name("Saúde").as_ref().is_some() {
        let child = stack.child_by_name("Saúde").unwrap();
        stack.remove(&child);
    } else if stack.child_by_name("Alimentação").as_ref().is_some() {
        let child = stack.child_by_name("Alimentação").unwrap();
        stack.remove(&child);
    }

    stack.add_titled(&grid_infos, Some(&infos[0].nature), &infos[0].nature);
    grid_infos
}
