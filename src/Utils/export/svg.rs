use crate::model::Debtor::Debtor;
use charts_rs::{Box, HorizontalBarChart, LineChart, PieChart, Series, THEME_GRAFANA};
use std::{env, io::Write};

pub fn to_svg(year: i16, data: Vec<(String, Vec<f32>)>, debtors: Vec<Debtor>) {
    let mut series: Vec<Series> = vec![];

    for data in data {
        series.push((data.0.trim(), data.1.clone()).into());
    }

    p_pie(series.clone());
    p_line(year, series.clone());

    series = vec![];
    for debtor in debtors {
        series.push((debtor.get_name().trim(), vec![debtor.get_value()]).into());
    }

    p_bar(series);
}

fn p_line(year: i16, series: Vec<Series>) {
    let mut plot = LineChart::new_with_theme(
        series,
        vec![
            "Jan".to_string(),
            "Fev".to_string(),
            "Mar".to_string(),
            "Abr".to_string(),
            "Mai".to_string(),
            "Jun".to_string(),
            "Jul".to_string(),
            "Ago".to_string(),
            "Set".to_string(),
            "Out".to_string(),
            "Nov".to_string(),
            "Dez".to_string(),
        ],
        THEME_GRAFANA,
    );

    plot.title_text = format!("{year} - Resumo");
    plot.title_font_size = 24.0;
    plot.legend_font_size = 16.0;

    plot.legend_margin = Some(Box {
        top: plot.title_font_size,
        bottom: 10.0,
        ..Default::default()
    });

    plot.y_axis_configs.push(plot.y_axis_configs[0].clone());
    plot.y_axis_configs[0].axis_formatter = Some("R$ {c}".to_string());

    plot.width = 1920.0;
    plot.height = 460.0;

    to_save(plot.svg().unwrap(), "plot_1B");
}
fn p_pie(series: Vec<Series>) {
    let mut plot = PieChart::new_with_theme(series, THEME_GRAFANA);

    plot.width = 800.0;
    plot.height = 600.0;

    to_save(plot.svg().unwrap(), "plot_1A");
}
fn p_bar(series: Vec<Series>) {
    let mut plot =
        HorizontalBarChart::new_with_theme(series, vec!["Total gasto".to_string()], THEME_GRAFANA);

    plot.width = 800.0;
    plot.height = 600.0;

    to_save(plot.svg().unwrap(), "plot_2A");
}

fn to_save(svg: String, name: &str) {
    use std::fs::File;

    let mut file = File::create(format!("{}/{name}.svg", env::temp_dir().display())).unwrap();
    let _ = file.write_all(svg.as_bytes());
}
