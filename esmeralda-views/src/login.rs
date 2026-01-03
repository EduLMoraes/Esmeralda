use crate::*;
use eframe::egui;
use egui::{Color32, RichText, Shadow, TextureOptions, Vec2};
use esmeralda_services::UserService;
use std::sync::Arc;
use esmeralda_entities::user::User;

pub struct LoginScreen {
    username: String,
    password: String,
    error_msg: Option<String>,
    user_service: Arc<dyn UserService>,
}

impl LoginScreen {
    pub fn new(user_service: Arc<dyn UserService>) -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            error_msg: None,
            user_service,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<LoginAction> {
        let mut action = None;

        let bg_image = egui::include_image!("../assets/img/background-login.png");
        let screen_rect = ui.ctx().content_rect();

        ui.painter().image(
            bg_image
                .load(
                    ui.ctx(),
                    TextureOptions::default(),
                    egui::SizeHint::Size {
                        width: 1,
                        height: 1,
                        maintain_aspect_ratio: false,
                    },
                )
                .map(|i| i.texture_id())
                .unwrap_or_default()
                .unwrap_or_default(),
            screen_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        let dark_green_btn = Color32::from_rgb(1, 50, 32);
        let gold_text = Color32::from_rgb(212, 175, 55);
        let input_bg = Color32::from_gray(230);
        let input_text = Color32::BLACK;

        ui.vertical_centered(|ui| {
            ui.add_space(60.0);

            ui.heading(
                RichText::new("Esmeralda")
                    .size(40.0)
                    .strong()
                    .color(SUCCESS_GREEN),
            );

            ui.label(RichText::new("💲").color(GOLD_ACCENT));

            ui.label(
                RichText::new("Gestão Financeira Inteligente")
                    .size(16.0)
                    .color(Color32::WHITE),
            );

            ui.add_space(40.0);

            egui::Frame::new()
                .fill(Color32::BLACK)
                .shadow(Shadow {
                    offset: [0, 0],
                    blur: 45,
                    spread: 0,
                    color: GOLD_ACCENT,
                })
                .inner_margin(30.0)
                .corner_radius(16.0)
                .show(ui, |ui| {
                    ui.set_max_width(320.0);

                    let mut visuals = ui.style().visuals.clone();
                    visuals.widgets.inactive.bg_fill = input_bg;
                    visuals.widgets.hovered.bg_fill = input_bg;
                    visuals.widgets.active.bg_fill = input_bg;
                    visuals.widgets.inactive.fg_stroke.color = input_text;
                    visuals.widgets.hovered.fg_stroke.color = input_text;
                    visuals.widgets.active.fg_stroke.color = input_text;
                    visuals.selection.stroke.color = input_text;
                    ui.ctx().set_visuals(visuals);

                    ui.vertical(|ui| {
                        ui.label(RichText::new("Usuário").color(Color32::WHITE).strong());
                        ui.add_space(2.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.username)
                                .desired_width(f32::INFINITY)
                                .margin(Vec2::splat(6.0))
                                .font(egui::TextStyle::Body),
                        );
                    });

                    ui.add_space(15.0);

                    ui.vertical(|ui| {
                        ui.label(RichText::new("Senha").color(Color32::WHITE).strong());
                        ui.add_space(2.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .desired_width(f32::INFINITY)
                                .margin(Vec2::splat(6.0)),
                        );
                    });

                    if let Some(error) = &self.error_msg {
                        ui.add_space(10.0);
                        ui.label(RichText::new(error).color(Color32::LIGHT_RED).small());
                    }

                    ui.add_space(30.0);

                    let btn_enter = egui::Button::new(
                        RichText::new("Entrar")
                            .size(22.0)
                            .color(gold_text)
                            .italics()
                            .family(egui::FontFamily::Proportional),
                    )
                    .fill(dark_green_btn)
                    .stroke(egui::Stroke::new(1.0, gold_text))
                    .min_size(egui::vec2(140.0, 45.0));

                    if ui.add(btn_enter).clicked() {
                        match self.user_service.login(&self.username, &self.password) {
                            Ok(user) => action = Some(LoginAction::Submit(user)),
                            Err(e) => self.error_msg = Some(e.to_string()),
                        }
                    }

                    ui.add_space(20.0);

                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Não possui uma conta? Registre-se")
                                    .color(Color32::WHITE)
                                    .underline(),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        action = Some(LoginAction::GoToRegister);
                    }
                });
        });

        action
    }
}

pub enum LoginAction {
    Submit(User),
    GoToRegister,
}
