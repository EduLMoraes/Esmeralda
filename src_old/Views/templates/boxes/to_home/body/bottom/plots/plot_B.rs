use super::*;
use crate::env;
use crate::gtk::Picture;

pub fn get_plot() -> Box {
    let box_plot = Box::new(Orientation::Horizontal, 0);
    box_plot.add_css_class("box_plots");

    let line = Picture::for_filename(format!("{}/plot_1B.svg", env::temp_dir().display()));
    line.add_css_class("plot_1B");
    line.set_hexpand(true);

    box_plot.append(&line);

    box_plot
}
