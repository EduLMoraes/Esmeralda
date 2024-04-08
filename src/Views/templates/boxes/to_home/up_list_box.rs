use gtk::ListBoxRow;

use super::*;

pub fn update_list(counts: &ListCount) {
    let list_box = get_list_box();
    list_box.remove_all();

    for count in &counts.list {
        list_box.append(&new_box_info(count));
    }
}
