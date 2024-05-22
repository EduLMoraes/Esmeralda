use charts_rs::{Box, LineChart, Series, THEME_GRAFANA};
use std::{env, io::Write};

pub fn to_svg(year: i16, data: Vec<(String, Vec<f32>)>) {
    let mut series_list: Vec<Series> = vec![];

    for data in data {
        series_list.push((data.0.trim(), data.1.clone()).into());
    }

    let mut bar_chart = LineChart::new_with_theme(
        series_list,
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

    bar_chart.title_text = format!("{year} - Resumo");
    bar_chart.title_font_size = 24.0;
    bar_chart.legend_font_size = 16.0;

    bar_chart.legend_margin = Some(Box {
        top: bar_chart.title_font_size,
        bottom: 10.0,
        ..Default::default()
    });

    bar_chart
        .y_axis_configs
        .push(bar_chart.y_axis_configs[0].clone());
    bar_chart.y_axis_configs[0].axis_formatter = Some("R$ {c}".to_string());

    bar_chart.width = 1080.0;
    bar_chart.height = 300.0;

    to_save(bar_chart.svg().unwrap());
}

fn to_save(svg: String) {
    use std::fs::File;

    let mut file = File::create(format!("{}/plot_1A.svg", env::temp_dir().display())).unwrap();
    let _ = file.write_all(svg.as_bytes());
}
