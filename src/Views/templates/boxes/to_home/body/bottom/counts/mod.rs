use super::*;
use crate::control::GLOBAL_COUNTS;
use crate::model::list::*;
use crate::model::Debtor::Debtor;
use crate::model::Info::Info;
use gtk::{DropDown, ScrolledWindow};

mod bottom_left;
mod bottom_right;
mod box_add;
mod box_debtor;
mod box_group;
mod box_home;
mod box_info;
mod box_pay;

mod grids;
use grids::get_grid_infos;

pub use bottom_left::*;
pub use bottom_right::*;
pub use box_add::*;
pub use box_debtor::*;
pub use box_group::*;
pub use box_home::*;
pub use box_info::*;
pub use box_pay::*;

pub fn box_count() -> Box {
    let box_count = Box::new(Orientation::Horizontal, 0);

    box_count.append(&left());
    box_count.append(&right());

    box_count.add_css_class("box_bottom_b");

    box_count
}
