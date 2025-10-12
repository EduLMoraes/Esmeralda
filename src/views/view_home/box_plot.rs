use crate::{utils, views::view_home::IsBoxView};
use gtk::{prelude::*, Box, Orientation, Picture};
use std::env;

pub struct Plot {}
impl Plot {
    const TITLE: &'static str = "plots";

    pub fn new() -> Self {
        Self {}
    }

    pub fn get_plot_a(&self) -> Box {
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

    pub fn get_plot_b(&self) -> Box {
        let box_plot = Box::new(Orientation::Horizontal, 0);
        box_plot.add_css_class("box_plots");

        let line = Picture::for_filename(format!("{}/plot_1B.svg", env::temp_dir().display()));
        line.add_css_class("plot_1B");
        line.set_hexpand(true);

        box_plot.append(&line);

        box_plot
    }
}

impl IsBoxView for Plot {
    fn get_box(&mut self) -> Box {
        let box_graph = Box::new(Orientation::Vertical, 0);
        box_graph.append(&self.get_plot_a());
        box_graph.append(&self.get_plot_b());

        box_graph
    }

    fn get_title(&self) -> &'static str {
        Self::TITLE
    }
}
