use super::*;

#[allow(static_mut_refs)]
pub fn get_list_box() -> &'static mut ListBox {
    let list_box = unsafe { LISTBOX.get_mut() };

    let box_list_count = match list_box {
        Some(list_box) => list_box,
        None => unsafe {
            LISTBOX = OnceLock::from(ListBox::new());
            LISTBOX.get_mut().unwrap()
        },
    };

    box_list_count
}
