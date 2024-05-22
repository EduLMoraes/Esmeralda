use super::*;
use crate::env;
use crate::gtk::Picture;

pub fn get_plot() -> Box {
    let box_plot = Box::new(Orientation::Horizontal, 0);
    box_plot.add_css_class("box_plots");

    let img = Picture::for_filename(format!("{}/plot_1A.svg", env::temp_dir().display()));
    img.add_css_class("plot_1A");
    img.set_hexpand(true);

    box_plot.append(&img);

    box_plot
}
