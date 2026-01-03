use eframe::egui;

pub struct RegisterScreen {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub error_msg: Option<String>,
}

impl Default for RegisterScreen {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            error_msg: None,
        }
    }
}

impl RegisterScreen {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<RegisterAction> {
        let mut action = None;

        ui.vertical_centered(|ui| {
            ui.add_space(40.0);

            ui.heading(
                egui::RichText::new("Criar Conta")
                    .size(28.0)
                    .strong()
                    .color(egui::Color32::from_rgb(80, 200, 120)),
            );
            ui.label("Comece a organizar suas finanças hoje");
            ui.add_space(25.0);

            egui::Frame::new()
                .fill(ui.visuals().faint_bg_color)
                .corner_radius(10.0)
                .inner_margin(25.0)
                .show(ui, |ui| {
                    ui.set_max_width(300.0);

                    ui.label("Nome Completo");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.name)
                            .hint_text("Como quer ser chamado?")
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(10.0);

                    ui.label("E-mail");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.email)
                            .hint_text("seu@email.com")
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(10.0);

                    ui.label("Senha");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.password)
                            .password(true)
                            .desired_width(f32::INFINITY),
                    );
                    ui.add_space(10.0);

                    ui.label("Confirmar Senha");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.confirm_password)
                            .password(true)
                            .desired_width(f32::INFINITY),
                    );

                    if let Some(error) = &self.error_msg {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(error)
                                .color(egui::Color32::LIGHT_RED)
                                .small(),
                        );
                    }

                    ui.add_space(25.0);

                    let btn_register =
                        egui::Button::new(egui::RichText::new("Criar Conta Esmeralda").strong())
                            .min_size(egui::vec2(ui.available_width(), 35.0));

                    if ui.add(btn_register).clicked() {
                        if self.validate() {
                            action = Some(RegisterAction::Submit);
                        }
                    }

                    ui.add_space(15.0);

                    if ui.link("Já tenho uma conta. Entrar").clicked() {
                        action = Some(RegisterAction::GoToLogin);
                    }
                });
        });

        action
    }

    fn validate(&mut self) -> bool {
        if self.name.is_empty() || self.email.is_empty() || self.password.is_empty() {
            self.error_msg = Some("Todos os campos são obrigatórios.".to_string());
            false
        } else if self.password != self.confirm_password {
            self.error_msg = Some("As senhas não coincidem.".to_string());
            false
        } else if self.password.len() < 6 {
            self.error_msg = Some("A senha deve ter pelo menos 6 caracteres.".to_string());
            false
        } else {
            self.error_msg = None;
            true
        }
    }
}

pub enum RegisterAction {
    Submit,
    GoToLogin,
}
