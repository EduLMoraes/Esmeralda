use super::*;
pub fn get_plot() -> Box {
    let box_plot = Box::new(Orientation::Horizontal, 0);

    let img = Image::from_file("./plot_1A.svg");
    img.add_css_class("plot_1A");

    box_plot.append(&img);

    box_plot
}
