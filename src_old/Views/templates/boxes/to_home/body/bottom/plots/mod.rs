use super::*;

#[allow(non_snake_case)]
mod plot_A;
#[allow(non_snake_case)]
mod plot_B;

pub fn box_graph() -> Box {
    let box_graph = Box::new(Orientation::Vertical, 0);
    box_graph.append(&plot_A::get_plot());
    box_graph.append(&plot_B::get_plot());

    box_graph
}
