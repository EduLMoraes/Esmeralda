use super::*;

use crate::model::Count::Count;
use crate::model::Debtor::Debtor;
use crate::model::List::*;
use crate::std::sync::OnceLock;
use gtk::{DropDown, ListBox, ScrolledWindow};

static mut LISTBOX: OnceLock<ListBox> = OnceLock::new();
static mut BOXHOME: OnceLock<Box> = OnceLock::new();

mod get_list_box;
mod up_list_box;
use get_list_box::get_list_box;
use up_list_box::*;

mod body;
mod menu_left;

pub use body::*;
pub use menu_left::*;
