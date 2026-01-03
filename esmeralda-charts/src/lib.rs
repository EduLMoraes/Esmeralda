use charts_rs::{Box, CanvasError, HorizontalBarChart, LineChart, PieChart, Series, THEME_GRAFANA};
use chrono::Datelike;
use esmeralda_entities::debt::Debt;
use std::collections::HashMap;
use std::fs::File;
use std::{
    env,
    io::{Error as IoError, Write},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChartError {
    #[error("Failed to generate chart: {0}")]
    Generate(String),
    #[error("Failed to save chart: {0}")]
    Save(String),
}

pub trait ChartGenerator {
    fn generate_all_charts(&self, year: i32, debts: &[Debt]) -> Result<(), ChartError>;
}

pub struct ChartGeneratorImpl;

impl ChartGenerator for ChartGeneratorImpl {
    fn generate_all_charts(&self, year: i32, debts: &[Debt]) -> Result<(), ChartError> {
        let (monthly_summary, debtor_summary) = self.process_data(year, debts);
        self.p_line(year, monthly_summary.clone())?;
        self.p_pie(monthly_summary)?;
        self.p_bar(debtor_summary)?;
        Ok(())
    }
}

impl ChartGeneratorImpl {
    fn process_data(&self, year: i32, debts: &[Debt]) -> (Vec<Series>, Vec<Series>) {
        let mut monthly_summary: HashMap<String, Vec<f32>> = HashMap::new();
        let mut debtor_summary: HashMap<String, f64> = HashMap::new();

        for debt in debts.iter().filter(|d| d.date_start.year() == year) {
            let month = debt.date_start.month0() as usize;
            let nature_str = format!("{:?}", debt.nature);
            let entry = monthly_summary.entry(nature_str).or_insert(vec![0.0; 12]);
            entry[month] += debt.value as f32;

            *debtor_summary
                .entry(debt.debtor.name.clone())
                .or_insert(0.0) += debt.value;
        }

        let monthly_series = monthly_summary
            .into_iter()
            .map(|(name, data)| (name.as_str(), data).into())
            .collect();
        let debtor_series = debtor_summary
            .into_iter()
            .map(|(name, value)| (name.as_str(), vec![value as f32]).into())
            .collect();

        (monthly_series, debtor_series)
    }

    fn p_line(&self, year: i32, series: Vec<Series>) -> Result<(), ChartError> {
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
        plot.title_text = format!("{} - Resumo", year);
        plot.title_font_size = 24.0;
        plot.legend_font_size = 16.0;
        plot.legend_margin = Some(Box {
            top: plot.title_font_size,
            bottom: 10.0,
            ..Default::default()
        });
        plot.y_axis_configs.push(plot.y_axis_configs[0].clone());
        plot.y_axis_configs[0].axis_formatter = Some("R$ {c}".to_string());
        plot.width = 1080.0;
        plot.height = 460.0;

        self.to_save(
            plot.svg()
                .map_err(|e: CanvasError| ChartError::Generate(e.to_string()))?,
            "plot_1B",
        )
    }

    fn p_pie(&self, series: Vec<Series>) -> Result<(), ChartError> {
        let mut plot = PieChart::new_with_theme(series, THEME_GRAFANA);
        plot.width = 800.0;
        plot.height = 600.0;
        self.to_save(
            plot.svg()
                .map_err(|e: CanvasError| ChartError::Generate(e.to_string()))?,
            "plot_1A",
        )
    }

    fn p_bar(&self, series: Vec<Series>) -> Result<(), ChartError> {
        let mut plot = HorizontalBarChart::new_with_theme(
            series,
            vec!["Total gasto".to_string()],
            THEME_GRAFANA,
        );
        plot.width = 800.0;
        plot.height = 600.0;
        self.to_save(
            plot.svg()
                .map_err(|e: CanvasError| ChartError::Generate(e.to_string()))?,
            "plot_2A",
        )
    }

    fn to_save(&self, svg: String, name: &str) -> Result<(), ChartError> {
        let mut file = File::create(format!("{}/{}.svg", env::temp_dir().display(), name))
            .map_err(|e: IoError| ChartError::Save(e.to_string()))?;
        file.write_all(svg.as_bytes())
            .map_err(|e: IoError| ChartError::Save(e.to_string()))
    }
}
