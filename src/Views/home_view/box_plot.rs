use super::*;

mod config;
mod counts;
mod investments;
mod plots;
pub use config::*;
pub use counts::*;
pub use investments::*;
pub use plots::*;

pub struct Plot {}
impl Plot {
    pub fn box_graph() -> Box {
        let box_graph = Box::new(Orientation::Vertical, 0);
        box_graph.append(&plot_A::get_plot_a());
        box_graph.append(&plot_B::get_plot_b());

        box_graph
    }
    pub fn get_plot_a() -> Box {
        let box_plot = Box::new(Orientation::Horizontal, 0);
        box_plot.add_css_class("box_plot_A");
        box_plot.set_halign(gtk::Align::Center);

        let pie = Picture::for_filename(format!("{}/plot_1A.svg", env::temp_dir().display()));
        pie.add_css_class("plot_1A");
        pie.set_vexpand(true);

        let bar = Picture::for_filename(format!("{}/plot_2A.svg", env::temp_dir().display()));
        bar.add_css_class("plot_2A");
        bar.set_vexpand(true);

        box_plot.append(&pie);
        box_plot.append(&bar);

        box_plot
    }

    pub fn get_plot_b() -> Box {
        let box_plot = Box::new(Orientation::Horizontal, 0);
        box_plot.add_css_class("box_plots");

        let line = Picture::for_filename(format!("{}/plot_1B.svg", env::temp_dir().display()));
        line.add_css_class("plot_1B");
        line.set_hexpand(true);

        box_plot.append(&line);

        box_plot
    }
}
