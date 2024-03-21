use super::*;

pub fn box_graph() -> Box {
    let box_graph = Box::new(Orientation::Horizontal, 0);

    box_graph.append(&Label::new(Some("Graficos")));

    box_graph.add_css_class("box_bottom_b");

    box_graph
}
