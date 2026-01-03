use crate::*;
use eframe::egui;
use egui::{Color32, RichText, Shadow, TextureOptions, Vec2};

pub struct LoginScreen {
    username: String,
    password: String,
    error_msg: Option<String>,
}

impl Default for LoginScreen {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            error_msg: None,
        }
    }
}

impl LoginScreen {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<LoginAction> {
        let mut action = None;

        // 1. Renderizar Imagem de Fundo (Cobrindo toda a tela)
        // Usamos ui.put para colocar a imagem "por trás" do layout normal, ocupando o rect da tela
        let bg_image = egui::include_image!("../assets/img/background-login.png");
        let screen_rect = ui.ctx().content_rect();

        // Cria uma camada de pintura para o fundo
        ui.painter().image(
            bg_image
                .load(ui.ctx(), TextureOptions::default(), egui::SizeHint::Size{width: 1, height: 1, maintain_aspect_ratio: false })
                .map(|i| i.texture_id())
                .unwrap_or_default()
                .unwrap_or_default(),
            screen_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        // Cores personalizadas conforme pedido
        let dark_green_btn = Color32::from_rgb(1, 50, 32); // Verde bem escuro
        let gold_text = Color32::from_rgb(212, 175, 55);
        let input_bg = Color32::from_gray(230); // Cinza claro
        let input_text = Color32::BLACK;

        // 2. Layout Centralizado
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);

            // Título
            ui.heading(
                RichText::new("Esmeralda")
                    .size(40.0)
                    .strong()
                    .color(SUCCESS_GREEN), // Ou use dark_green_btn se preferir
            );

            ui.label(RichText::new("💲").color(GOLD_ACCENT));

            ui.label(
                RichText::new("Gestão Financeira Inteligente")
                    .size(16.0)
                    .color(Color32::WHITE), // Texto branco para contrastar com a imagem de fundo
            );

            ui.add_space(40.0);

            // 3. Container do Formulário
            egui::Frame::new()
                .fill(Color32::BLACK) // Fundo escuro translúcido para ler em cima da imagem
                .shadow(Shadow { offset: [0, 0], blur: 45, spread: 0, color: GOLD_ACCENT })
                .inner_margin(30.0)
                .corner_radius(16.0)
                .show(ui, |ui| {
                    ui.set_max_width(320.0);

                    // --- Estilização dos Inputs (Fundo Cinza, Letra Preta) ---
                    let mut visuals = ui.style().visuals.clone();
                    visuals.widgets.inactive.bg_fill = input_bg;
                    visuals.widgets.hovered.bg_fill = input_bg;
                    visuals.widgets.active.bg_fill = input_bg;
                    visuals.widgets.inactive.fg_stroke.color = input_text;
                    visuals.widgets.hovered.fg_stroke.color = input_text;
                    visuals.widgets.active.fg_stroke.color = input_text;
                    visuals.selection.stroke.color = input_text; // Cor do cursor
                    ui.ctx().set_visuals(visuals);

                    // Campo de Usuário
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Usuário").color(Color32::WHITE).strong());
                        ui.add_space(2.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.username)
                                .desired_width(f32::INFINITY)
                                .margin(Vec2::splat(6.0)) // Padding de ~6px (maior que 3px pedido)
                                .font(egui::TextStyle::Body),
                        );
                    });

                    ui.add_space(15.0);

                    // Campo de Senha
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Senha").color(Color32::WHITE).strong());
                        ui.add_space(2.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.password)
                                .password(true)
                                .desired_width(f32::INFINITY)
                                .margin(Vec2::splat(6.0)), // Padding interno
                        );
                    });

                    // Mensagem de Erro
                    if let Some(error) = &self.error_msg {
                        ui.add_space(10.0);
                        ui.label(RichText::new(error).color(Color32::LIGHT_RED).small());
                    }

                    ui.add_space(30.0);

                    // 4. Botão Entrar (Verde Escuro, Dourado, Itálico)
                    let btn_enter = egui::Button::new(
                        RichText::new("Entrar")
                            .size(22.0)
                            .color(gold_text)
                            .italics()
                            // Tenta usar uma fonte serifada se disponível para lembrar Times New Roman
                            .family(egui::FontFamily::Proportional),
                    )
                    .fill(dark_green_btn)
                    .stroke(egui::Stroke::new(1.0, gold_text))
                    .min_size(egui::vec2(140.0, 45.0));

                    if ui.add(btn_enter).clicked() {
                        if self.validate().is_ok() {
                            action = Some(LoginAction::Submit);
                        }
                    }

                    ui.add_space(20.0);

                    // 5. Link Registrar-se (Transparente)
                    if ui
                        .add(
                            egui::Button::new(
                                RichText::new("Não possui uma conta? Registre-se")
                                    .color(Color32::WHITE) // Branco para ler no fundo escuro
                                    .underline(),
                            )
                            .frame(false), // Fundo transparente
                        )
                        .clicked()
                    {
                        action = Some(LoginAction::GoToRegister);
                    }
                });
        });

        action
    }

    fn validate(&mut self) -> Result<(), String> {
        if self.username.len() < 1 || self.password.len() < 4 {
            self.error_msg = Some("Usuário/Senha inválidos (min. 4 caracteres)".to_string());
            return Err("Erro".to_string());
        }
        Ok(())
    }
}

pub enum LoginAction {
    Submit,
    GoToRegister,
}
