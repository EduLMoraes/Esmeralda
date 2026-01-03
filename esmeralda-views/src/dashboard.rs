use crate::*;
use chrono::{Datelike, NaiveDate};
use eframe::egui;
use egui::{Color32, Frame, RichText, Stroke, Ui};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints};
use esmeralda_entities::debt::{Debt, NatureDebt};
use std::{
    collections::{BTreeMap, HashMap},
    f32,
};

#[derive(Default)]
pub struct DashboardView;

impl DashboardView {
    pub fn ui(&self, ui: &mut Ui, data: &[Debt]) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(10.0);
            ui.set_max_size(egui::Vec2::new(ui.available_width(), 200.0));

            self.render_investment_section(ui, data);

            ui.add_space(20.0);

            self.render_cash_flow_section(ui, data);

            ui.add_space(20.0);

            self.render_expense_distribution_section(ui, data);

            ui.add_space(50.0);
        });
    }

    fn render_investment_section(&self, ui: &mut Ui, data: &[Debt]) {
        ui.heading(RichText::new("📈 Evolução Patrimonial").color(GOLD_ACCENT));
        ui.add_space(5.0);

        self.chart_container(ui, 300.0, |ui| {
            let line = self.prepare_investment_data(data);

            Plot::new("investment_plot")
                .show_background(false)
                .show_axes([true, true])
                .show_grid([true, true])
                .y_axis_formatter(|val, _range| format!("R$ {:.0}", val.value))
                .x_axis_formatter(|val, _range| {
                    let date =
                        NaiveDate::from_num_days_from_ce_opt(val.value as i32).unwrap_or_default();
                    date.format("%d/%m/%y").to_string()
                })
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });
    }

    fn render_cash_flow_section(&self, ui: &mut Ui, data: &[Debt]) {
        ui.heading(RichText::new("📊 Fluxo de Caixa Mensal").color(GOLD_ACCENT));
        ui.add_space(5.0);

        let (income_chart, expense_chart, x_labels) = self.prepare_cash_flow_data(data);

        self.chart_container(ui, 300.0, |ui| {
            Plot::new("cash_flow_plot")
                .show_background(false)
                .legend(Legend::default().position(egui_plot::Corner::RightTop))
                .y_axis_formatter(|val, _range| format!("R$ {:.0}", val.value))
                .x_axis_formatter(move |val, _range| {
                    let index = val.value.round() as usize;
                    if index < x_labels.len() {
                        x_labels[index].clone()
                    } else {
                        String::new()
                    }
                })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(income_chart);
                    plot_ui.bar_chart(expense_chart);
                });
        });
    }

    fn render_expense_distribution_section(&self, ui: &mut Ui, data: &[Debt]) {
        ui.heading(RichText::new("🍩 Despesas por Categoria").color(GOLD_ACCENT));
        ui.add_space(5.0);

        let (bar_chart, y_labels) = self.prepare_expense_distribution_data(data);

        self.chart_container(ui, 50.0, |ui| {
            Plot::new("expense_dist_plot")
                .show_background(false)
                .show_axes([true, true])
                .x_axis_formatter(|val, _range| format!("R$ {:.0}", val.value))
                .y_axis_formatter(move |val, _range| {
                    let index = val.value.round() as usize;
                    if index < y_labels.len() {
                        y_labels[index].clone()
                    } else {
                        String::new()
                    }
                })
                .show(ui, |plot_ui| {
                    plot_ui.bar_chart(bar_chart);
                });
        });
    }

    fn chart_container<F>(&self, ui: &mut Ui, height: f32, add_contents: F)
    where
        F: FnOnce(&mut Ui),
    {
        Frame::new()
            .fill(CARD_BG)
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.set_min_height(height);
                add_contents(ui);
            });
    }

    fn prepare_investment_data(&self, data: &[Debt]) -> Line<'_> {
        let mut investments: Vec<&Debt> = data
            .iter()
            .filter(|d| matches!(d.nature, NatureDebt::Investment))
            .collect();

        investments.sort_by_key(|d| d.date_start);

        let mut accumulated = 0.0;
        let points: PlotPoints = investments
            .iter()
            .map(|d| {
                accumulated += d.value.abs();

                [d.date_start.num_days_from_ce() as f64, accumulated]
            })
            .collect();

        Line::new("Patrimônio Acumulado", points)
            .color(GOLD_ACCENT)
            .width(2.0)
            .fill(0.1)
            .name("Patrimônio Acumulado")
    }

    fn prepare_cash_flow_data(&self, data: &[Debt]) -> (BarChart, BarChart, Vec<String>) {
        let mut monthly_data: BTreeMap<(i32, u32), (f64, f64)> = BTreeMap::new();

        for d in data {
            let key = (d.date_start.year(), d.date_start.month());
            let entry = monthly_data.entry(key).or_insert((0.0, 0.0));

            if matches!(d.nature, NatureDebt::Incoming) {
                entry.0 += d.value.abs();
            } else if !matches!(d.nature, NatureDebt::Investment) {
                entry.1 += d.value.abs();
            }
        }

        let mut income_bars = Vec::new();
        let mut expense_bars = Vec::new();
        let mut x_labels = Vec::new();
        let mut index = 0.0;

        for ((year, month), (inc, exp)) in monthly_data {
            income_bars.push(
                Bar::new(index - 0.2, inc)
                    .width(0.3)
                    .fill(SUCCESS_GREEN)
                    .name("Receitas"),
            );
            expense_bars.push(
                Bar::new(index + 0.2, exp)
                    .width(0.3)
                    .fill(DEBT_RED)
                    .name("Despesas"),
            );

            let month_str = match month {
                1 => "Jan",
                2 => "Fev",
                3 => "Mar",
                4 => "Abr",
                5 => "Mai",
                6 => "Jun",
                7 => "Jul",
                8 => "Ago",
                9 => "Set",
                10 => "Out",
                11 => "Nov",
                12 => "Dez",
                _ => "",
            };
            x_labels.push(format!("{}/{}", month_str, year % 100));
            index += 1.0;
        }

        (
            BarChart::new("Entradas", income_bars)
                .name("Entradas")
                .color(SUCCESS_GREEN),
            BarChart::new("Saídas", expense_bars)
                .name("Saídas")
                .color(DEBT_RED),
            x_labels,
        )
    }

    fn prepare_expense_distribution_data(&self, data: &[Debt]) -> (BarChart, Vec<String>) {
        let mut category_totals: HashMap<String, f64> = HashMap::new();

        for d in data {
            if matches!(d.nature, NatureDebt::Incoming | NatureDebt::Investment) {
                continue;
            }

            let category_name = match &d.nature {
                NatureDebt::Home => "Casa",
                NatureDebt::Health => "Saúde",
                NatureDebt::Transport => "Transporte",
                NatureDebt::Food => "Alimentação",
                NatureDebt::Other(s) => {
                    if s.is_empty() {
                        "Outros"
                    } else {
                        s
                    }
                }
                _ => "Geral",
            }
            .to_string();

            *category_totals.entry(category_name).or_insert(0.0) += d.value.abs();
        }

        let mut sorted_cats: Vec<(String, f64)> = category_totals.into_iter().collect();

        sorted_cats.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut bars = Vec::new();
        let mut y_labels = Vec::new();

        for (i, (name, val)) in sorted_cats.iter().enumerate() {
            bars.push(
                Bar::new(i as f64, *val)
                    .horizontal()
                    .width(0.6)
                    .fill(self.get_color_for_category(name))
                    .name(name.clone()),
            );
            y_labels.push(name.clone());
        }

        (
            BarChart::new("Distribuição de Despesas", bars).color(GOLD_ACCENT),
            y_labels,
        )
    }

    fn get_color_for_category(&self, cat: &str) -> Color32 {
        match cat {
            "Casa" => Color32::from_rgb(100, 149, 237),
            "Alimentação" => Color32::from_rgb(255, 160, 122),
            "Saúde" => Color32::from_rgb(144, 238, 144),
            "Transporte" => Color32::from_rgb(255, 215, 0),
            _ => Color32::LIGHT_GRAY,
        }
    }
}
