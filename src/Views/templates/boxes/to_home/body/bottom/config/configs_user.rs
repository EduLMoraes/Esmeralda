use super::*;
use crate::apis::get_quote;
use gtk::{Adjustment, CheckButton, SpinButton};

pub fn get_box() -> Box {
    let box_index = Box::new(Orientation::Horizontal, 2);
    box_index.add_css_class("box_config");

    

    let grid = Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);



    box_index.append(&grid);
    box_index
}
