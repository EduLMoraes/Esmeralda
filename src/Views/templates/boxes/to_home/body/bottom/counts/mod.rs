use super::*;

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
    // let box_count =  unsafe{ BOXHOME.get_mut() };
    // let box_count = match box_count{
    //     Some(box_c) => { 
    //         let left_w = box_c.first_child().unwrap();
    //         let right_w = box_c.last_child().unwrap();

    //         box_c.remove(&left_w);
    //         box_c.remove(&right_w);

    //         box_c.append(&left());
    //         box_c.append(&right());

    //         box_c
    //      },
    //     None => {
    //         unsafe { 
    //             BOXHOME = OnceLock::from(Box::new(Orientation::Horizontal, 0));
    //             let tmp = BOXHOME.get_mut().unwrap();
    //             tmp.append(&left());
    //             tmp.append(&right());

    //             tmp.add_css_class("box_bottom_b");
                
    //             tmp
    //         }
    //     }
    // };

    let box_count = Box::new(Orientation::Horizontal, 0);

    box_count.append(&left());
    box_count.append(&right());


    box_count
}