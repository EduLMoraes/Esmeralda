use crate::{add_account::AddAccountScreen, *};
use eframe::egui;
use egui::{Color32, RichText, Stroke, Vec2};
use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
struct EntryData {
    pub value: f64,
    pub nature: Nature,
    pub title: String,
    pub is_paid: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Nature {
    Home,
    Recept,
    Health,
}

impl Display for Nature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
    Details(String),
}

pub struct HomeScreen {
    selected_tab: Tab,
    add_account_modal: AddAccountScreen,
    is_adding_account: bool,
    mode: ViewMode,
    data: HashMap<String, Vec<EntryData>>,
    search_query: String,
}

impl Default for HomeScreen {
    fn default() -> Self {
        let mut data = HashMap::new();

        data.insert(
            "Casa".into(),
            vec![EntryData {
                value: -750.75,
                nature: Nature::Home,
                title: "Aluguel".to_string(),
                is_paid: false,
            }],
        );
        data.insert(
            "Receitas".into(),
            vec![EntryData {
                value: 3000.25,
                nature: Nature::Recept,
                title: "Salário".to_string(),
                is_paid: true,
            }],
        );
        data.insert(
            "Saúde".into(),
            vec![EntryData {
                value: -100.0,
                nature: Nature::Health,
                title: "Farmácia".to_string(),
                is_paid: false,
            }],
        );

        if let Some(debt) = data.get_mut("Casa") {
            for i in 0..200 {
                debt.push(EntryData {
                    value: -150.00,
                    nature: Nature::Home,
                    title: format!("Internet {}", i),
                    is_paid: false,
                });
            }
        }

        Self {
            selected_tab: Tab::Overview,
            add_account_modal: AddAccountScreen::default(),
            is_adding_account: false,
            mode: ViewMode::Natures,
            data,
            search_query: String::new(),
        }
    }
}

impl HomeScreen {
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
                    ui.selectable_value(
                        &mut self.selected_tab,
                        Tab::Investments,
                        "💱 Investimentos",
                    );
                    ui.selectable_value(&mut self.selected_tab, Tab::Calculator, "🖩 Calculadora");
                    ui.selectable_value(&mut self.selected_tab, Tab::Settings, "⚙ Configurações");
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
                        ui.heading("Dashboard");
                    }
                    Tab::Investments => {
                        ui.heading("Investimentos");
                    }
                    Tab::Calculator => {
                        ui.heading("Calculadora");
                    }
                    Tab::Settings => {
                        ui.heading("Configurações");
                    }
                });
        });

        if self.is_adding_account {
            self.add_account_modal.ui(ctx, &mut self.is_adding_account);
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
                                .hint_text("Buscar natureza ou categoria...")
                                .frame(false)
                                .desired_width(f32::INFINITY),
                        );
                        if !self.search_query.is_empty() {
                            if ui.button(RichText::new("✖").small()).clicked() {
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
                    let min_box_width = 150.0;
                    let spacing = 10.0;
                    let columns = ((available_width) / (min_box_width + spacing))
                        .floor()
                        .max(1.0);
                    let box_width =
                        (available_width - (spacing * (columns - 1.0)) - 30.0) / columns;

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
                let btn_w = 160.0;
                let total_btn_w = btn_w * 2.0 + 20.0;
                if available_width > total_btn_w {
                    ui.add_space((available_width - total_btn_w) / 2.0);
                }

                if ui
                    .add(
                        egui::Button::new(RichText::new("+ Adicionar Receita").strong())
                            .fill(SUCCESS_GREEN)
                            .min_size(Vec2::new(btn_w, 35.0)),
                    )
                    .clicked()
                {
                    self.is_adding_account = true;
                }
                ui.add_space(10.0);
                if ui
                    .add(
                        egui::Button::new(RichText::new("- Adicionar Dívida").strong())
                            .fill(DEBT_RED)
                            .min_size(Vec2::new(btn_w, 35.0)),
                    )
                    .clicked()
                {
                    self.is_adding_account = true;
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

        let mut recent_items: Vec<(&String, &EntryData)> = Vec::new();
        for (category, entries) in &self.data {
            for entry in entries {
                recent_items.push((category, entry));
            }
        }
        recent_items.reverse();

        let scroll_height = 90.0;
        let scroll_rect = ui.available_rect_before_wrap();

        egui::ScrollArea::horizontal().hscroll(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(12.0);

                for (category, entry) in recent_items.iter().take(30) {
                    self.draw_mini_card(ui, category, entry);
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

    fn draw_mini_card(&self, ui: &mut egui::Ui, category: &str, entry: &EntryData) {
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
                        ui.label(RichText::new(category).size(10.0).color(Color32::GRAY));
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
                let mut display_list: Vec<(String, Vec<EntryData>)> = Vec::new();

                for (category_name, entries) in &self.data {
                    let filtered_entries: Vec<EntryData> = entries
                        .iter()
                        .filter(|entry| {
                            let match_title = entry.title.to_lowercase().contains(&query);
                            let match_value = entry.value.to_string().contains(&query);
                            let match_category = category_name.to_lowercase().contains(&query);
                            match_title || match_value || match_category
                        })
                        .cloned()
                        .collect();

                    if !filtered_entries.is_empty() {
                        display_list.push((category_name.clone(), filtered_entries));
                    }
                }

                display_list.sort_by(|a, b| a.0.cmp(&b.0));

                if display_list.is_empty() {
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

                egui::Grid::new("responsive_grid")
                    .spacing(Vec2::splat(spacing))
                    .min_col_width(card_width)
                    .show(ui, |ui| {
                        for (i, (name, entries)) in display_list.iter().enumerate() {
                            self.draw_nature_card(
                                ui,
                                name,
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

            ViewMode::Details(nature_name) => {
                self.render_details_view(ui, &nature_name);
            }
        }
    }

    fn draw_nature_card(
        &mut self,
        ui: &mut egui::Ui,
        name: &str,
        data: &[EntryData],
        width: f32,
        itens: u64,
    ) {
        let total: f64 = data.iter().map(|a| a.value).sum();
        let value_color = if total >= 0.0 {
            SUCCESS_GREEN
        } else {
            DEBT_RED
        };

        ui.allocate_ui(Vec2::new(width, 150.0), |ui| {
            egui::Frame::new()
                .fill(CARD_BG)
                .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.2)))
                .corner_radius(10.0)
                .inner_margin(15.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let img = match data.first().unwrap().nature {
                                Nature::Health => {
                                    egui::include_image!("../assets/icon/info_icon/health.png")
                                }
                                Nature::Home => {
                                    egui::include_image!("../assets/icon/info_icon/home.png")
                                }
                                Nature::Recept => {
                                    egui::include_image!("../assets/icon/info_icon/recept.png")
                                }
                            };
                            ui.add(
                                egui::Image::new(img)
                                    .fit_to_exact_size(Vec2::splat(ICON_SIZE))
                                    .corner_radius(4.0),
                            );

                            ui.vertical(|ui| {
                                ui.label(RichText::new(name).strong().size(16.0));
                                ui.label(
                                    RichText::new(format!("{} itens", itens))
                                        .color(Color32::GRAY)
                                        .size(12.0),
                                );
                            });
                        });

                        ui.add_space(20.0);
                        ui.label(
                            RichText::new(format!("R$ {:.2}", total))
                                .color(value_color)
                                .size(26.0)
                                .strong(),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                            if ui
                                .link(RichText::new("+ Detalhes").color(GOLD_ACCENT))
                                .clicked()
                            {
                                self.mode = ViewMode::Details(name.to_string());
                            }
                        });
                    });
                });
        });
    }

    fn render_details_view(&mut self, ui: &mut egui::Ui, nature_name: &String) {
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new("⬅ Voltar").color(GOLD_ACCENT))
                .clicked()
            {
                self.mode = ViewMode::Natures;
            }
            ui.add_space(10.0);
            ui.heading(RichText::new(format!("Detalhes: {}", nature_name)).color(Color32::WHITE));
        });
        ui.add_space(20.0);

        let query = self.search_query.to_lowercase();
        let mut display_list: Vec<EntryData> = Vec::new();

        if let Some(items) = self.data.get(nature_name) {
            for entry in items {
                let match_title = entry.title.to_lowercase().contains(&query);
                let match_value = entry.value.to_string().contains(&query);
                if match_title || match_value {
                    display_list.push(entry.clone());
                }
            }
        }

        if display_list.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label(RichText::new("Nenhum item encontrado.").color(Color32::GRAY));
            });
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for item in display_list {
                self.draw_count_row(ui, &item);
                ui.add_space(10.0);
            }
        });
    }

    fn draw_count_row(&self, ui: &mut egui::Ui, item: &EntryData) {
        egui::Frame::new()
            .fill(CARD_BG)
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(&item.title).strong().size(16.0));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(10.0);
                        if item.is_paid {
                            ui.label(RichText::new("PAGO").color(SUCCESS_GREEN).strong());
                        } else {
                            let (btn_text, btn_color) = if item.nature == Nature::Recept {
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
                            {}
                        }

                        ui.add_space(20.0);
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

                        if ui.small_button("✏").on_hover_text("Editar").clicked() {}
                    });
                });
            });
    }

    fn calculate_totals(&self) -> (f64, f64, f64) {
        let mut rec = 0.0;
        let mut div = 0.0;
        for d in self.data.values() {
            for ed in d {
                if ed.value > 0.0 {
                    rec += ed.value;
                } else {
                    div += ed.value.abs();
                }
            }
        }
        (rec, div, rec - div)
    }
}
