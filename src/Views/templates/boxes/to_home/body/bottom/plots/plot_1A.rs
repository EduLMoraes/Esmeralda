use super::*;
use crate::env;

pub fn get_plot() -> Box {
    let box_plot = Box::new(Orientation::Horizontal, 0);

    let img = Image::from_file(format!("{}/plot_1A.svg", env::temp_dir().display()));
    img.add_css_class("plot_1A");

    box_plot.append(&img);

    box_plot
}
