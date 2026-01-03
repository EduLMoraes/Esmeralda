#![allow(unused)]
use crate::{
    add_debt::AddDebtScreen, add_incoming::AddIncomingScreen, calculator::CalculatorScreen,
    dashboard::DashboardView, *,
};
use chrono::{Datelike, Months, NaiveDate};
use eframe::egui;
use egui::{Color32, RichText, Stroke, Vec2};
use esmeralda_debt::{DebtService};
use esmeralda_entities::debt::{Debt, NatureDebt};
use std::collections::HashMap;
use uuid::Uuid;
use esmeralda_services::UserService;
use esmeralda_entities::user::User;
use std::sync::Arc;

#[derive(PartialEq)]
enum Tab {
    Overview,
    Dashboard,
    Investments,
    Calculator,
    Settings,
}

#[derive(PartialEq, Clone)]
enum ViewMode {
    Natures,
    Details(NatureDebt),
}

enum RowAction {
    None,
    Pay(usize),
    ToggleEdit(Uuid),
    Delete(usize),
}

pub struct HomeScreen {
    selected_tab: Tab,
    add_debt_modal: AddDebtScreen,
    is_adding_debt: bool,
    add_incoming_modal: AddIncomingScreen,
    is_adding_incoming: bool,
    mode: ViewMode,
    data: Vec<Debt>,
    search_query: String,
    debt_service: Arc<dyn DebtService>,
    user_service: Arc<dyn UserService>,
    user: Option<User>,
    editing_id: Option<Uuid>,
    calculator_view: CalculatorScreen,
}

impl HomeScreen {
    pub fn new(user_service: Arc<dyn UserService>, debt_service: Arc<dyn DebtService>) -> Self {
        let data = debt_service.get_all("").unwrap_or_default();

        Self {
            selected_tab: Tab::Overview,
            add_debt_modal: AddDebtScreen::new(None),
            is_adding_debt: false,
            add_incoming_modal: AddIncomingScreen::new(None),
            is_adding_incoming: false,
            mode: ViewMode::Natures,
            data,
            search_query: String::new(),
            debt_service,
            user_service,
            user: None,
            editing_id: None,
            calculator_view: CalculatorScreen::default(),
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.data = self.debt_service.get_all(&user.id.to_string()).unwrap_or_default();
        self.user = Some(user);
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(
                        RichText::new("Esmeralda")
                            .size(28.0)
                            .strong()
                            .color(GOLD_ACCENT),
                    );
                    ui.label(
                        RichText::new("Gestão Financeira")
                            .color(Color32::GRAY)
                            .small(),
                    );
                    ui.add_space(20.0);
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.add_space(10.0);
                    ui.selectable_value(&mut self.selected_tab, Tab::Overview, "🏠 Inicio");
                    ui.selectable_value(&mut self.selected_tab, Tab::Dashboard, "📊 Dashboard");

                    ui.selectable_value(&mut self.selected_tab, Tab::Calculator, "🖩 Calculadora");
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Sair").color(DEBT_RED))
                                .min_size(Vec2::new(180.0, 30.0)),
                        )
                        .clicked()
                    {}
                    ui.separator();
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_top_bar(ui);
            ui.add_space(15.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match self.selected_tab {
                    Tab::Overview => self.render_overview(ui),
                    Tab::Dashboard => {
                        let dashboard = DashboardView::default();
                        dashboard.ui(ui, &self.data);
                    }
                    Tab::Investments => {
                        ui.heading("Investimentos");
                    }
                    Tab::Calculator => {
                        self.calculator_view.ui(ui);
                    }
                    Tab::Settings => {
                        ui.heading("Configurações");
                    }
                });
        });

        if self.is_adding_debt {
            self.add_debt_modal.user = self.user.clone();
            if let Some(new_debt) = self.add_debt_modal.ui(ctx, &mut self.is_adding_debt) {
                let _ = self.debt_service.insert(new_debt);
                self.data = self.debt_service.get_all(&self.user.as_ref().unwrap().id.to_string()).unwrap_or_default();
            }
        }
        if self.is_adding_incoming {
            self.add_incoming_modal.user = self.user.clone();
            if let Some(new_incoming) = self.add_incoming_modal.ui(ctx, &mut self.is_adding_incoming)
            {
                let _ = self.debt_service.insert(new_incoming);
                self.data = self.debt_service.get_all(&self.user.as_ref().unwrap().id.to_string()).unwrap_or_default();
            }
        }
    }

    fn render_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            egui::Frame::new()
                .fill(CARD_BG)
                .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
                .corner_radius(8.0)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🔍").color(GOLD_ACCENT));
                        ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text("Buscar...")
                                .frame(false)
                                .desired_width(f32::INFINITY),
                        );
                        if !self.search_query.is_empty() {
                            if ui.button("✖").clicked() {
                                self.search_query.clear();
                            }
                        }
                    });
                });
        });
    }

    fn render_overview(&mut self, ui: &mut egui::Ui) {
        let (receita, divida, rendimento) = self.calculate_totals();
        let available_width = ui.available_width();

        egui::Frame::new()
            .fill(SLATE_BG)
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let columns = ((available_width) / 160.0).floor().max(1.0);
                    let box_width = (available_width - (10.0 * (columns - 1.0)) - 30.0) / columns;
                    self.stat_box(ui, "RECEITAS", receita, SUCCESS_GREEN, box_width);
                    self.stat_box(ui, "DÍVIDAS", divida, DEBT_RED, box_width);
                    self.stat_box(
                        ui,
                        "RENDIMENTO",
                        rendimento,
                        if rendimento >= 0.0 {
                            SUCCESS_GREEN
                        } else {
                            DEBT_RED
                        },
                        box_width,
                    );
                });
            });

        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::new("+ Receita").strong())
                            .fill(SUCCESS_GREEN)
                            .min_size(Vec2::new(160.0, 35.0)),
                    )
                    .clicked()
                {
                    self.is_adding_incoming = true;
                }
                ui.add_space(10.0);
                if ui
                    .add(
                        egui::Button::new(RichText::new("- Dívida").strong())
                            .fill(DEBT_RED)
                            .min_size(Vec2::new(160.0, 35.0)),
                    )
                    .clicked()
                {
                    self.is_adding_debt = true;
                }
            });
        });

        ui.add_space(30.0);
        ui.separator();
        ui.add_space(20.0);

        self.render_cards(ui, available_width);

        if let ViewMode::Natures = self.mode {
            if self.search_query.is_empty() {
                self.render_recent_entries(ui);
            }
        }
    }

    fn render_recent_entries(&self, ui: &mut egui::Ui) {
        ui.add_space(20.0);
        ui.label(
            RichText::new("Adicionados Recentemente")
                .strong()
                .size(16.0)
                .color(Color32::LIGHT_GRAY),
        );
        ui.add_space(5.0);

        let mut recent_items: Vec<&Debt> = self.data.iter().collect();
        recent_items.reverse();

        let scroll_height = 90.0;
        let scroll_rect = ui.available_rect_before_wrap();

        egui::ScrollArea::horizontal().hscroll(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(12.0);

                for entry in recent_items.iter().take(30) {
                    self.draw_mini_card(ui, entry);
                }
            });
        });

        let fade_width = 80.0;
        let fade_rect = egui::Rect::from_min_size(
            egui::Pos2::new(ui.max_rect().right() - fade_width, scroll_rect.top()),
            Vec2::new(fade_width, scroll_height - 11.0),
        );

        let mut mesh = egui::Mesh::default();
        let transparent =
            Color32::from_rgba_premultiplied(SLATE_BG.r(), SLATE_BG.g(), SLATE_BG.b(), 0);
        let solid = SLATE_BG;

        mesh.colored_vertex(fade_rect.left_top(), transparent);
        mesh.colored_vertex(fade_rect.right_top(), solid);
        mesh.colored_vertex(fade_rect.right_bottom(), solid);
        mesh.colored_vertex(fade_rect.left_bottom(), transparent);

        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);

        ui.painter().add(mesh);
    }

    fn draw_mini_card(&self, ui: &mut egui::Ui, entry: &Debt) {
        ui.allocate_ui(Vec2::new(150.0, 75.0), |ui| {
            egui::Frame::new()
                .fill(CARD_BG)
                .stroke(Stroke::new(1.0, Color32::from_rgb(50, 50, 50)))
                .corner_radius(8.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                        let color = if entry.value >= 0.0 {
                            SUCCESS_GREEN
                        } else {
                            DEBT_RED
                        };
                        ui.label(RichText::new("●").size(8.0).color(color));
                        ui.label(
                            RichText::new(format!("{:?}", entry.nature))
                                .size(10.0)
                                .color(Color32::GRAY),
                        );
                    });

                    ui.label(
                        RichText::new(&entry.title)
                            .strong()
                            .size(13.0)
                            .color(Color32::WHITE),
                    );

                    let val_color = if entry.value >= 0.0 {
                        SUCCESS_GREEN
                    } else {
                        DEBT_RED
                    };
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                        ui.label(
                            RichText::new(format!("R$ {:.2}", entry.value))
                                .color(val_color)
                                .size(14.0),
                        );
                    });
                });
        });
    }

    fn stat_box(&self, ui: &mut egui::Ui, label: &str, val: f64, color: Color32, width: f32) {
        ui.allocate_ui(Vec2::new(width, 80.0), |ui| {
            egui::Frame::new()
                .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.3)))
                .corner_radius(6.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(label).size(10.0).color(Color32::GRAY));
                        ui.heading(
                            RichText::new(format!("R$ {:.2}", val.abs()))
                                .color(color)
                                .size(22.0),
                        );
                    });
                });
        });
    }

    fn render_cards(&mut self, ui: &mut egui::Ui, available_width: f32) {
        match self.mode.clone() {
            ViewMode::Natures => {
                let query = self.search_query.to_lowercase();
                let mut display_map: HashMap<NatureDebt, Vec<Debt>> = HashMap::new();
                for entry in &self.data {
                    if entry.title.to_lowercase().contains(&query)
                        || format!("{:?}", entry.nature)
                            .to_lowercase()
                            .contains(&query)
                    {
                        display_map
                            .entry(entry.nature.clone())
                            .or_default()
                            .push(entry.clone());
                    }
                }

                if display_map.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(RichText::new("Nenhum item encontrado.").color(Color32::GRAY));
                    });
                    return;
                }

                let min_card_width = 220.0;
                let spacing = 15.0;
                let columns = ((available_width + spacing) / (min_card_width + spacing))
                    .floor()
                    .max(1.0) as usize;
                let card_width =
                    (available_width - (spacing * (columns as f32 - 1.0))) / columns as f32;

                let mut keys: Vec<_> = display_map.keys().cloned().collect();

                keys.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));

                egui::Grid::new("responsive_grid")
                    .spacing(Vec2::splat(spacing))
                    .min_col_width(card_width)
                    .show(ui, |ui| {
                        for (i, nature) in keys.iter().enumerate() {
                            let entries = display_map.get(&nature).unwrap();
                            self.draw_nature_card(
                                ui,
                                nature,
                                entries,
                                card_width,
                                entries.len() as u64,
                            );
                            if (i + 1) % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
            }

            ViewMode::Details(nature) => {
                self.render_details_view(ui, &nature);
            }
        }
    }

    fn draw_nature_card(
        &mut self,
        ui: &mut egui::Ui,
        nature: &NatureDebt,
        data: &[Debt],
        width: f32,
        itens: u64,
    ) {
        ui.allocate_ui(Vec2::new(width, 150.0), |ui| {
            egui::Frame::new()
                .fill(CARD_BG)
                .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.2)))
                .corner_radius(10.0)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("{:?}", nature)).strong().size(16.0));
                        ui.label(
                            RichText::new(format!("{} itens", itens))
                                .small()
                                .color(Color32::GRAY),
                        );
                        let total: f64 = data.iter().map(|d| d.value).sum();
                        ui.add_space(10.0);
                        ui.label(RichText::new(format!("R$ {:.2}", total)).size(24.0).color(
                            if total >= 0.0 {
                                SUCCESS_GREEN
                            } else {
                                DEBT_RED
                            },
                        ));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                            if ui
                                .link(RichText::new("+ Detalhes").color(GOLD_ACCENT))
                                .clicked()
                            {
                                self.mode = ViewMode::Details(nature.clone());
                            }
                        });
                    });
                });
        });
    }

    fn render_details_view(&mut self, ui: &mut egui::Ui, nature: &NatureDebt) {
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new("⬅ Voltar").color(GOLD_ACCENT))
                .clicked()
            {
                self.mode = ViewMode::Natures;
                self.editing_id = None;
            }
            ui.add_space(10.0);
            ui.heading(RichText::new(format!("Detalhes: {:?}", nature)).color(Color32::WHITE));
        });
        ui.add_space(20.0);

        let query = self.search_query.to_lowercase();

        let indices: Vec<usize> = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, entry)| {
                let match_nature = entry.nature == *nature;
                let match_text = entry.title.to_lowercase().contains(&query);
                match_nature && match_text
            })
            .map(|(idx, _)| idx)
            .collect();

        if indices.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(RichText::new("Nenhum item encontrado.").color(Color32::GRAY));
            });
            return;
        }

        let mut action_to_perform = RowAction::None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for idx in indices {
                if let Some(item) = self.data.get_mut(idx) {
                    let is_editing = self.editing_id == Some(item.id);

                    let action = Self::draw_count_row(ui, item, is_editing, idx);

                    if let RowAction::None = action {
                    } else {
                        action_to_perform = action;
                    }
                }
                ui.add_space(10.0);
            }
        });

        match action_to_perform {
            RowAction::Pay(idx) => {
                if let Some(item) = self.data.get_mut(idx) {
                    let _ = self.debt_service.pay_installment(item);
                }
            }
            RowAction::ToggleEdit(uuid) => {
                if self.editing_id == Some(uuid) {
                    self.editing_id = None;
                } else {
                    self.editing_id = Some(uuid);
                }
            }
            RowAction::Delete(idx) => {
                self.data.remove(idx);
            }
            RowAction::None => {}
        }
    }

    fn draw_count_row(
        ui: &mut egui::Ui,
        item: &mut Debt,
        is_editing: bool,
        idx: usize,
    ) -> RowAction {
        let mut action = RowAction::None;

        let frame_stroke = if is_editing {
            Stroke::new(1.0, GOLD_ACCENT)
        } else {
            Stroke::new(1.0, Color32::from_rgb(60, 60, 60))
        };

        let t_title = item.title.clone();
        let t_date_start = item.date_start;
        let t_date_end = item.date_end;
        let t_paid_installments = item.paid_installments;
        let t_installments = item.installments;
        let t_value = item.value;

        let tooltip_ui = move |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Resumo da Dívida")
                        .strong()
                        .color(GOLD_ACCENT),
                );
                ui.separator();

                let months_paid = t_paid_installments as u32;
                let last_paid_date = t_date_start
                    .checked_add_months(Months::new(months_paid))
                    .unwrap_or(t_date_start);

                ui.label(format!("Início: {}", t_date_start.format("%d/%m/%Y")));
                ui.label(format!("Fim Previsto: {}", t_date_end.format("%d/%m/%Y")));
                ui.label(
                    RichText::new(format!(
                        "Última parcela paga: {}",
                        last_paid_date.format("%d/%m/%Y")
                    ))
                    .color(SUCCESS_GREEN),
                );

                ui.add_space(5.0);

                let total_val = t_value.abs();
                let total_parcelas = if t_installments == 0 {
                    1
                } else {
                    t_installments
                };

                let val_parcela = total_val / (total_parcelas as f64);
                let total_pago = val_parcela * (t_paid_installments as f64);
                let saldo_devedor = total_val - total_pago;

                ui.horizontal(|ui| {
                    ui.label("Total Pago:");
                    ui.label(RichText::new(format!("R$ {:.2}", total_pago)).color(SUCCESS_GREEN));
                });
                ui.horizontal(|ui| {
                    ui.label("Em Aberto:");
                    ui.label(RichText::new(format!("R$ {:.2}", saldo_devedor)).color(DEBT_RED));
                });
            });
        };

        egui::Frame::new()
            .fill(CARD_BG)
            .stroke(frame_stroke)
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if !is_editing {
                            ui.label(RichText::new(&t_title).strong().size(16.0))
                                .on_hover_ui(tooltip_ui);
                        } else {
                            ui.horizontal(|ui| {
                                ui.label("Título:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut item.title)
                                        .desired_width(120.0),
                                );
                            });
                        }

                        if !is_editing {
                            ui.label(
                                RichText::new(format!(
                                    "Parcelas: {} / {}",
                                    item.paid_installments, item.installments
                                ))
                                .small()
                                .color(Color32::GRAY),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "Início: {}",
                                    item.date_start.format("%d/%m/%Y")
                                ))
                                .small()
                                .color(Color32::GRAY),
                            );
                        } else {
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.label("Val:");
                                ui.add(egui::DragValue::new(&mut item.value).speed(1.0));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Pag:");
                                ui.add(
                                    egui::DragValue::new(&mut item.paid_installments).speed(0.1),
                                );
                                ui.label("Tot:");
                                ui.add(egui::DragValue::new(&mut item.installments).speed(0.1));
                            });

                            ui.horizontal(|ui| {
                                ui.label("Cat:");

                                egui::ComboBox::from_id_salt(format!("nature_combo_{}", item.id))
                                    .selected_text(format!("{:?}", item.nature))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Home,
                                            "Casa",
                                        );
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Health,
                                            "Saúde",
                                        );
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Food,
                                            "Alimentação",
                                        );
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Transport,
                                            "Transporte",
                                        );
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Investment,
                                            "Investimento",
                                        );
                                        ui.selectable_value(
                                            &mut item.nature,
                                            NatureDebt::Incoming,
                                            "Receita",
                                        );
                                    });
                            });

                            ui.horizontal(|ui| {
                                let mut d = item.date_start.day();
                                let mut m = item.date_start.month();
                                let mut y = item.date_start.year();
                                ui.label("Início:");
                                ui.add(egui::DragValue::new(&mut d).range(1..=31));
                                ui.label("/");
                                ui.add(egui::DragValue::new(&mut m).range(1..=12));
                                ui.label("/");
                                ui.add(egui::DragValue::new(&mut y).range(2000..=2100));
                                if let Some(ndt) = NaiveDate::from_ymd_opt(y, m, d) {
                                    item.date_start = ndt;
                                }
                            });

                            ui.label(
                                RichText::new(format!(
                                    "Fim (Calc): {}",
                                    item.date_end.format("%d/%m/%Y")
                                ))
                                .small()
                                .color(Color32::GRAY),
                            );
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(RichText::new("🗑").color(DEBT_RED))
                            .on_hover_text("Excluir")
                            .clicked()
                        {
                            action = RowAction::Delete(idx);
                        }

                        ui.add_space(5.0);

                        let icon = if is_editing { "💾" } else { "✏" };
                        let icon_color = if is_editing {
                            SUCCESS_GREEN
                        } else {
                            GOLD_ACCENT
                        };

                        let edit_btn = egui::Button::new(RichText::new(icon).color(icon_color));
                        if ui
                            .add(edit_btn)
                            .on_hover_text(if is_editing { "Salvar" } else { "Editar" })
                            .clicked()
                        {
                            action = RowAction::ToggleEdit(item.id);
                        }

                        ui.add_space(10.0);

                        if !item.status {
                            let (btn_text, btn_color) = if let NatureDebt::Incoming = item.nature {
                                ("Receber", SUCCESS_GREEN)
                            } else {
                                ("Pagar", DEBT_RED)
                            };
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(btn_text).color(Color32::BLACK),
                                    )
                                    .fill(btn_color),
                                )
                                .clicked()
                            {
                                action = RowAction::Pay(idx);
                            }
                        } else {
                            ui.label(RichText::new("PAGO").color(SUCCESS_GREEN).strong());
                        }

                        ui.add_space(20.0);

                        if !is_editing {
                            let val_color = if item.value >= 0.0 {
                                SUCCESS_GREEN
                            } else {
                                DEBT_RED
                            };
                            ui.label(
                                RichText::new(format!("R$ {:.2}", item.value))
                                    .color(val_color)
                                    .size(18.0)
                                    .strong(),
                            );
                        }
                    });
                });
            });

        action
    }
    fn calculate_totals(&self) -> (f64, f64, f64) {
        let mut rec = 0.0;
        let mut div = 0.0;
        for d in &self.data {
            if d.value > 0.0 {
                rec += d.value;
            } else {
                div += d.value.abs();
            }
        }
        (rec, div, rec - div)
    }
}