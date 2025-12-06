use super::*;
use crate::env;
use crate::gtk::Picture;

pub fn get_plot() -> Box {
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
