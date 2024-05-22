use super::*;

#[allow(non_snake_case)]
mod plot_1A;

pub fn box_graph() -> Box {
    let box_graph = Box::new(Orientation::Vertical, 0);
    box_graph.append(&plot_1A::get_plot());

    box_graph
}
