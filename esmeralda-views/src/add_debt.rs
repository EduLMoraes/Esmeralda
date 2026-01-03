use crate::*;
use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;
use egui::Vec2;
use egui::{Color32, RichText, Stroke};
use esmeralda_entities::debt::{Debt, NatureDebt};
use uuid::Uuid;
use esmeralda_entities::user::User;
use esmeralda_entities::people::People;

pub struct AddDebtScreen {
    pub title: String,
    pub value: String,
    pub nature: NatureDebt,
    pub installments: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub is_paid: bool,
    pub error_msg: Option<String>,
    pub user: Option<User>,
}

impl AddDebtScreen {
    pub fn new(user: Option<User>) -> Self {
        Self {
            title: String::new(),
            value: String::new(),
            nature: NatureDebt::Other("".to_string()),
            installments: "1".to_string(),
            description: String::new(),
            start_date: Local::now().naive_local().date(),
            is_paid: false,
            error_msg: None,
            user,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Option<Debt> {
        let mut result = None;
        let mut close_modal = false;

        let frame = egui::Frame::window(&ctx.style())
            .fill(SLATE_BG)
            .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.5)))
            .inner_margin(20.0)
            .corner_radius(8.0)
            .shadow(egui::Shadow::default());

        egui::Window::new("Nova Dívida")
            .frame(frame)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .title_bar(false)
            .show(ctx, |ui| {
                ui.set_width(340.0);

                let mut visuals = ui.style().visuals.clone();
                visuals.widgets.inactive.bg_fill = CARD_BG;
                visuals.widgets.active.bg_fill = CARD_BG;
                visuals.widgets.hovered.bg_fill = CARD_BG.gamma_multiply(1.2);
                visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, Color32::WHITE);
                visuals.widgets.active.fg_stroke = Stroke::new(1.0, GOLD_ACCENT);
                visuals.selection.stroke = Stroke::new(1.0, GOLD_ACCENT);
                ui.ctx().set_visuals(visuals);

                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new("Adicionar Dívida")
                            .color(GOLD_ACCENT)
                            .size(20.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(RichText::new("❌").small()).clicked() {
                            close_modal = true;
                        }
                    });
                });
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(15.0);

                ui.label(RichText::new("Título").strong().color(Color32::WHITE));
                ui.add(
                    egui::TextEdit::singleline(&mut self.title)
                        .hint_text("Ex: Cartão Nubank")
                        .desired_width(f32::INFINITY)
                        .margin(Vec2::splat(4.0)),
                );
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Valor (R$)").strong().color(Color32::WHITE));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.value)
                                .desired_width(150.0)
                                .margin(Vec2::splat(4.0)),
                        );
                    });

                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        ui.label(RichText::new("Parcelas").strong().color(Color32::WHITE));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.installments)
                                .desired_width(150.0)
                                .margin(Vec2::splat(4.0)),
                        );
                    });
                });
                ui.add_space(10.0);

                ui.label(
                    RichText::new("Data da 1ª Parcela")
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.horizontal(|ui| {
                    let mut day = self.start_date.day();
                    let mut month = self.start_date.month();
                    let mut year = self.start_date.year();

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
                        self.start_date = new_date;
                    }
                });

                let num_installments = self.installments.parse::<u32>().unwrap_or(1);

                let end_date = self
                    .start_date
                    .checked_add_months(chrono::Months::new(num_installments - 1))
                    .unwrap_or(self.start_date);

                ui.label(
                    RichText::new(format!("Término previsto: {}", end_date.format("%d/%m/%Y")))
                        .small()
                        .color(Color32::GRAY),
                );

                ui.add_space(10.0);

                ui.label(RichText::new("Categoria").strong().color(Color32::WHITE));
                egui::ComboBox::from_id_salt("nature_combo")
                    .selected_text(RichText::new(format!("{:?}", self.nature)).color(GOLD_ACCENT))
                    .width(ui.available_width())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.nature, NatureDebt::Health, "Saúde");
                        ui.selectable_value(&mut self.nature, NatureDebt::Home, "Casa");
                        ui.selectable_value(&mut self.nature, NatureDebt::Transport, "Transporte");
                        ui.selectable_value(&mut self.nature, NatureDebt::Food, "Alimentação");
                        ui.selectable_value(
                            &mut self.nature,
                            NatureDebt::Investment,
                            "Investimento",
                        );

                        ui.selectable_value(
                            &mut self.nature,
                            NatureDebt::Other("Outros".into()),
                            "Outros",
                        );
                    });

                ui.add_space(10.0);

                ui.checkbox(
                    &mut self.is_paid,
                    RichText::new("Esta dívida já está paga?").color(Color32::WHITE),
                );
                if !self.is_paid {
                    ui.label(
                        RichText::new("O valor será registrado como negativo.")
                            .small()
                            .color(DEBT_RED),
                    );
                }

                ui.add_space(10.0);

                ui.label(RichText::new("Descrição").strong().color(Color32::WHITE));
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
                            .color(DEBT_RED)
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
                            egui::Button::new(RichText::new("Cancelar").color(Color32::WHITE))
                                .fill(Color32::TRANSPARENT)
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .min_size(Vec2::new(100.0, 35.0)),
                        )
                        .clicked()
                    {
                        close_modal = true;
                    }

                    let btn_save = egui::Button::new(
                        RichText::new("Salvar Dívida")
                            .strong()
                            .color(Color32::BLACK),
                    )
                    .fill(GOLD_ACCENT)
                    .min_size(Vec2::new(ui.available_width(), 35.0));

                    if ui.add(btn_save).clicked() {
                        if let Some(user) = &self.user {
                            match self.validate_and_build(end_date, user) {
                                Ok(debt) => {
                                    result = Some(debt);
                                    close_modal = true;
                                    self.reset();
                                }
                                Err(e) => self.error_msg = Some(e),
                            }
                        }
                    }
                });
            });

        if close_modal {
            *is_open = false;
        }

        result
    }

    fn validate_and_build(&self, end_date: NaiveDate, user: &User) -> Result<Debt, String> {
        if self.title.trim().is_empty() {
            return Err("O título é obrigatório.".to_string());
        }

        let raw_val = self
            .value
            .replace(',', ".")
            .parse::<f64>()
            .map_err(|_| "Valor inválido.".to_string())?;

        let inst = self
            .installments
            .parse::<u16>()
            .map_err(|_| "Parcelas inválidas.".to_string())?;

        if inst == 0 {
            return Err("Mínimo de 1 parcela.".to_string());
        }

        let final_value = if !self.is_paid {
            -raw_val.abs()
        } else {
            -raw_val.abs()
        };

        Ok(Debt {
            id: Uuid::new_v4(),
            title: self.title.clone(),
            value: final_value,
            nature: self.nature.clone(),
            installments: inst,
            paid_installments: if self.is_paid { inst } else { 0 },
            description: self.description.clone(),
            status: self.is_paid,
            date_start: self.start_date,
            date_end: end_date,
            debtor: People { id: user.id, ..Default::default() },
            proof: None,
        })
    }

    fn reset(&mut self) {
        self.title.clear();
        self.value.clear();
        self.installments = "1".to_string();
        self.description.clear();
        self.start_date = Local::now().naive_local().date();
        self.is_paid = false;
        self.error_msg = None;
    }
}