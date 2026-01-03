use eframe::egui;

pub struct AddAccountScreen {
    pub name: String,
    pub balance: String, // Usamos String para o input e convertemos depois
    pub account_type: AccountType,
    pub error_msg: Option<String>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AccountType {
    Corrente,
    Poupanca,
    Investimento,
    Dinheiro,
}

impl Default for AddAccountScreen {
    fn default() -> Self {
        Self {
            name: String::new(),
            balance: "0.00".to_string(),
            account_type: AccountType::Corrente,
            error_msg: None,
        }
    }
}

impl AddAccountScreen {
    pub fn ui(&mut self, ctx: &egui::Context, is_open: &mut bool) -> Option<AccountData> {
        let mut result = None;
        let mut close_modal = false; // Variável auxiliar para evitar o conflito de borrow

        // Criamos a janela sem o .open() para evitar o borrow conflituoso
        egui::Window::new("Nova Conta Bancária")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_width(320.0);

                ui.label("Nome da Instituição/Conta");
                ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .hint_text("Ex: Banco Esmeralda")
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(10.0);

                ui.label("Saldo Inicial (R$)");
                ui.add(egui::TextEdit::singleline(&mut self.balance).desired_width(f32::INFINITY));

                ui.add_space(10.0);

                ui.label("Tipo de Conta");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.account_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.account_type,
                            AccountType::Corrente,
                            "Corrente",
                        );
                        ui.selectable_value(
                            &mut self.account_type,
                            AccountType::Poupanca,
                            "Poupança",
                        );
                        ui.selectable_value(
                            &mut self.account_type,
                            AccountType::Investimento,
                            "Investimento",
                        );
                        ui.selectable_value(
                            &mut self.account_type,
                            AccountType::Dinheiro,
                            "Dinheiro (Espécie)",
                        );
                    });

                if let Some(error) = &self.error_msg {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(error)
                            .color(egui::Color32::LIGHT_RED)
                            .small(),
                    );
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancelar").clicked() {
                        close_modal = true; // Alteramos a variável local
                    }

                    let btn_save = egui::Button::new(
                        egui::RichText::new("Salvar Conta")
                            .strong()
                            .color(egui::Color32::BLACK),
                    )
                    .fill(egui::Color32::from_rgb(80, 200, 120));

                    if ui.add(btn_save).clicked() {
                        if let Ok(val) = self.balance.replace(',', ".").parse::<f64>() {
                            if self.name.is_empty() {
                                self.error_msg = Some("Dê um nome à conta.".into());
                            } else {
                                result = Some(AccountData {
                                    name: self.name.clone(),
                                    balance: val,
                                    account_type: self.account_type,
                                });
                                close_modal = true; // Alteramos a variável local
                                self.reset();
                            }
                        } else {
                            self.error_msg = Some("Saldo inválido.".into());
                        }
                    }
                });
            });

        // Se o botão cancelar ou salvar foi clicado, atualizamos o estado externo
        if close_modal {
            *is_open = false;
        }

        result
    }

    fn reset(&mut self) {
        self.name.clear();
        self.balance = "0.00".to_string();
        self.error_msg = None;
    }
}

pub struct AccountData {
    pub name: String,
    pub balance: f64,
    pub account_type: AccountType,
}
