use super::*;

mod bottom_left;
mod bottom_right;
mod box_add;
mod box_debtor;
mod box_group;
mod box_home;
mod box_info;
mod box_month;
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
pub use box_month::*;
pub use box_pay::*;

#[allow(static_mut_refs)]
pub fn box_count() -> Box {
    let box_count = unsafe { BOXHOME.get_mut() };
    let box_count = match box_count {
        Some(box_c) => {
            let left_w = box_c.first_child().unwrap();

            box_c.remove(&left_w);
            box_c.prepend(&left());

            box_c
        }
        None => unsafe {
            BOXHOME = OnceLock::from(Box::new(Orientation::Horizontal, 0));
            let tmp = BOXHOME.get_mut().unwrap();
            tmp.append(&left());
            tmp.append(&right());

            tmp.add_css_class("box_bottom_b");

            tmp
        },
    };

    box_count.clone()
}
