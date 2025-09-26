use super::*;

mod count;
mod export;
mod graph;
mod import;
mod investments;

pub use count::*;
pub use export::*;
pub use graph::box_graph_link;
pub use import::*;
pub use investments::box_investments_link;

pub fn links(stack: &Stack) -> Box {
    let box_links = Box::new(Orientation::Vertical, 20);

    box_links.append(&box_count_link(stack));
    box_links.append(&box_graph_link(stack));
    box_links.append(&box_investments_link(stack));
    box_links.append(&box_export_link());
    box_links.append(&box_import_link());

    box_links.add_css_class("box_links_ml");
    box_links
}
