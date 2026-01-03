use crate::*;
use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;
use egui::{Color32, RichText, Stroke, Vec2};
use esmeralda_entities::debt::{Debt, NatureDebt};
use uuid::Uuid;

pub struct AddIncomingScreen {
    pub title: String,
    pub value: String,
    pub description: String,
    pub date: NaiveDate,
    pub error_msg: Option<String>,
}

impl Default for AddIncomingScreen {
    fn default() -> Self {
        Self {
            title: String::new(),
            value: String::new(),
            description: String::new(),
            date: Local::now().naive_local().date(),
            error_msg: None,
        }
    }
}

impl AddIncomingScreen {
    pub fn ui(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Option<Debt> {
        let mut result = None;
        let mut close_modal = false;

        let frame = egui::Frame::window(&ctx.style())
            .fill(SLATE_BG)
            .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.5)))
            .inner_margin(20.0)
            .corner_radius(8.0)
            .shadow(egui::Shadow::default());

        egui::Window::new("Nova Receita")
            .frame(frame)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .title_bar(false)
            .show(ctx, |ui| {
                ui.set_width(320.0);

                let mut visuals = ui.style().visuals.clone();
                visuals.widgets.inactive.bg_fill = CARD_BG;
                visuals.widgets.active.bg_fill = CARD_BG;
                visuals.widgets.hovered.bg_fill = CARD_BG.gamma_multiply(1.2);
                visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_WHITE);
                visuals.widgets.active.fg_stroke = Stroke::new(1.0, GOLD_ACCENT);
                visuals.selection.stroke = Stroke::new(1.0, GOLD_ACCENT);
                ui.ctx().set_visuals(visuals);

                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Nova Receita").color(GOLD_ACCENT).size(20.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("❌").small()).clicked() {
                            close_modal = true;
                        }
                    });
                });
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(15.0);

                ui.label(RichText::new("Título").strong().color(TEXT_WHITE));
                ui.add(
                    egui::TextEdit::singleline(&mut self.title)
                        .hint_text("Ex: Salário, Freelance...")
                        .desired_width(f32::INFINITY)
                        .margin(Vec2::splat(4.0)),
                );

                ui.add_space(10.0);

                ui.label(RichText::new("Valor (R$)").strong().color(TEXT_WHITE));
                ui.add(
                    egui::TextEdit::singleline(&mut self.value)
                        .hint_text("0.00")
                        .desired_width(f32::INFINITY)
                        .margin(Vec2::splat(4.0)),
                );

                ui.add_space(10.0);

                ui.label(
                    RichText::new("Data do Recebimento")
                        .strong()
                        .color(TEXT_WHITE),
                );
                ui.horizontal(|ui| {
                    let mut day = self.date.day();
                    let mut month = self.date.month();
                    let mut year = self.date.year();

                    ui.add(egui::DragValue::new(&mut day).range(1..=31).suffix(" Dia"));
                    ui.label("/");
                    ui.add(
                        egui::DragValue::new(&mut month)
                            .range(1..=12)
                            .suffix(" Mês"),
                    );
                    ui.label("/");
                    ui.add(
                        egui::DragValue::new(&mut year)
                            .range(2000..=2100)
                            .suffix(" Ano"),
                    );

                    if let Some(new_date) = NaiveDate::from_ymd_opt(year, month, day) {
                        self.date = new_date;
                    }
                });

                ui.add_space(10.0);

                ui.label(RichText::new("Descrição").strong().color(TEXT_WHITE));
                ui.add(
                    egui::TextEdit::multiline(&mut self.description)
                        .hint_text("Detalhes opcionais...")
                        .desired_width(f32::INFINITY)
                        .desired_rows(3)
                        .margin(Vec2::splat(4.0)),
                );

                if let Some(error) = &self.error_msg {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(format!("⚠ {}", error))
                            .color(Color32::LIGHT_RED)
                            .small(),
                    );
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 10.0;

                    if ui
                        .add(
                            egui::Button::new(RichText::new("Cancelar").color(TEXT_WHITE))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .min_size(Vec2::new(100.0, 35.0)),
                        )
                        .clicked()
                    {
                        close_modal = true;
                    }

                    let btn_save = egui::Button::new(
                        RichText::new("Confirmar Receita")
                            .strong()
                            .color(Color32::BLACK),
                    )
                    .fill(GOLD_ACCENT)
                    .min_size(Vec2::new(ui.available_width(), 35.0));

                    if ui.add(btn_save).clicked() {
                        match self.validate_and_build() {
                            Ok(debt) => {
                                result = Some(debt);
                                close_modal = true;
                                self.reset();
                            }
                            Err(e) => self.error_msg = Some(e),
                        }
                    }
                });
            });

        if close_modal {
            *is_open = false;
        }

        result
    }

    fn validate_and_build(&self) -> Result<Debt, String> {
        if self.title.trim().is_empty() {
            return Err("O título é obrigatório.".to_string());
        }

        let val = self
            .value
            .replace(',', ".")
            .parse::<f64>()
            .map_err(|_| "Valor inválido.".to_string())?;

        if val <= 0.0 {
            return Err("O valor deve ser maior que zero.".to_string());
        }

        Ok(Debt {
            id: Uuid::new_v4(),
            title: self.title.clone(),
            value: val.abs(),
            nature: NatureDebt::Incoming,
            installments: 1,
            paid_installments: 1,
            description: self.description.clone(),
            status: true,
            date_start: self.date,
            date_end: self.date,
            debtor: Default::default(),
            proof: None,
        })
    }

    fn reset(&mut self) {
        self.title.clear();
        self.value.clear();
        self.description.clear();
        self.date = Local::now().naive_local().date();
        self.error_msg = None;
    }
}
