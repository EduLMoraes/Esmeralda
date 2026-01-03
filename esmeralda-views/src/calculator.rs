use crate::*;
use eframe::egui;
use egui::{Color32, RichText, Stroke, Vec2};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CalculatorMode {
    Basic,
    CompoundInterest,
    SimpleInterest,
    MagicNumber,
    FuelCompare,
    FuelEfficiency,
    IncomeTax,
}

pub struct CalculatorScreen {
    mode: CalculatorMode,

    expression: String,
    history: Vec<(String, String)>,

    input_a: String,
    input_b: String,
    input_c: String,
    input_d: String,

    result_text: String,
    detail_text: String,
}

impl Default for CalculatorScreen {
    fn default() -> Self {
        Self {
            mode: CalculatorMode::Basic,
            expression: String::new(),
            history: Vec::new(),

            input_a: String::new(),
            input_b: String::new(),
            input_c: String::new(),
            input_d: String::new(),
            result_text: String::from("0"),
            detail_text: String::new(),
        }
    }
}

impl CalculatorScreen {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(
                RichText::new("🖩 Calculadora Financeira")
                    .color(GOLD_ACCENT)
                    .size(24.0),
            );
            ui.add_space(10.0);

            egui::ComboBox::from_id_salt("calc_mode_selector")
                .selected_text(RichText::new(self.get_mode_name()).color(GOLD_ACCENT))
                .width(250.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.mode, CalculatorMode::Basic, "Padrão (Básica)");
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::CompoundInterest,
                        "Juros Compostos",
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::SimpleInterest,
                        "Juros Simples",
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::MagicNumber,
                        "Número Mágico (Cotas)",
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::IncomeTax,
                        "Salário Líquido (IRRF)",
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::FuelCompare,
                        "Combustível (Álcool x Gasolina)",
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        CalculatorMode::FuelEfficiency,
                        "Combustível (Consumo Real)",
                    );
                });
        });

        ui.add_space(20.0);

        egui::Frame::new()
            .fill(CARD_BG)
            .stroke(Stroke::new(1.0, Color32::from_rgb(60, 60, 60)))
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.label(RichText::new("ℹ").size(20.0).color(GOLD_ACCENT));
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(self.get_description_title())
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new(self.get_description_text())
                                .small()
                                .color(Color32::GRAY),
                        );
                    });
                });
            });

        ui.add_space(20.0);

        egui::Frame::new()
            .fill(SLATE_BG)
            .inner_margin(10.0)
            .show(ui, |ui| match self.mode {
                CalculatorMode::Basic => self.render_basic(ui),
                CalculatorMode::CompoundInterest => self.render_compound(ui),
                CalculatorMode::SimpleInterest => self.render_simple(ui),
                CalculatorMode::MagicNumber => self.render_magic_number(ui),
                CalculatorMode::FuelCompare => self.render_fuel_compare(ui),
                CalculatorMode::FuelEfficiency => self.render_fuel_efficiency(ui),
                CalculatorMode::IncomeTax => self.render_ir(ui),
            });

        ui.add_space(20.0);

        ui.vertical_centered(|ui| {
            let btn = egui::Button::new(
                RichText::new("CALCULAR RESULTADO")
                    .strong()
                    .color(Color32::BLACK),
            )
            .fill(GOLD_ACCENT)
            .min_size(Vec2::new(200.0, 40.0));

            if ui.add(btn).clicked() {
                self.calculate();
            }

            ui.add_space(20.0);

            egui::Frame::new()
                .fill(CARD_BG)
                .stroke(Stroke::new(1.0, GOLD_ACCENT.gamma_multiply(0.5)))
                .corner_radius(12.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(300.0);
                    ui.label(RichText::new("Resultado").small().color(Color32::GRAY));
                    ui.heading(
                        RichText::new(&self.result_text)
                            .color(SUCCESS_GREEN)
                            .size(28.0),
                    );
                    if !self.detail_text.is_empty() {
                        ui.add_space(5.0);
                        ui.label(RichText::new(&self.detail_text).color(TEXT_WHITE));
                    }
                });
        });
    }

    fn render_basic(&mut self, ui: &mut egui::Ui) {
        self.handle_keyboard_input(ui);

        egui::Frame::new()
            .fill(Color32::BLACK)
            .stroke(Stroke::new(2.0, GOLD_ACCENT))
            .corner_radius(8.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical_centered_justified(|ui| {
                    if let Some(last) = self.history.last() {
                        ui.label(
                            RichText::new(format!("{} =", last.0))
                                .color(Color32::GRAY)
                                .size(14.0),
                        );
                    } else {
                        ui.label("");
                    }

                    ui.add(egui::Label::new(
                        RichText::new(if self.expression.is_empty() {
                            "0"
                        } else {
                            &self.expression
                        })
                        .size(40.0)
                        .color(TEXT_WHITE)
                        .family(egui::FontFamily::Monospace),
                    ));
                });
            });

        ui.add_space(20.0);

        let btn_size = Vec2::new(60.0, 50.0);

        egui::Grid::new("calc_keyboard")
            .spacing(Vec2::splat(10.0))
            .min_col_width(60.0)
            .show(ui, |ui| {
                if self.calc_btn(ui, "C", DEBT_RED, btn_size) {
                    self.expression.clear();
                    self.result_text = "0".into();
                }
                if self.calc_btn(ui, "(", SLATE_BG, btn_size) {
                    self.append_char('(');
                }
                if self.calc_btn(ui, ")", SLATE_BG, btn_size) {
                    self.append_char(')');
                }
                if self.calc_btn(ui, "⌫", DEBT_RED, btn_size) {
                    self.expression.pop();
                }
                ui.end_row();

                if self.calc_btn(ui, "7", CARD_BG, btn_size) {
                    self.append_char('7');
                }
                if self.calc_btn(ui, "8", CARD_BG, btn_size) {
                    self.append_char('8');
                }
                if self.calc_btn(ui, "9", CARD_BG, btn_size) {
                    self.append_char('9');
                }
                if self.calc_btn(ui, "/", GOLD_ACCENT, btn_size) {
                    self.append_char('/');
                }
                ui.end_row();

                if self.calc_btn(ui, "4", CARD_BG, btn_size) {
                    self.append_char('4');
                }
                if self.calc_btn(ui, "5", CARD_BG, btn_size) {
                    self.append_char('5');
                }
                if self.calc_btn(ui, "6", CARD_BG, btn_size) {
                    self.append_char('6');
                }
                if self.calc_btn(ui, "*", GOLD_ACCENT, btn_size) {
                    self.append_char('*');
                }
                ui.end_row();

                if self.calc_btn(ui, "1", CARD_BG, btn_size) {
                    self.append_char('1');
                }
                if self.calc_btn(ui, "2", CARD_BG, btn_size) {
                    self.append_char('2');
                }
                if self.calc_btn(ui, "3", CARD_BG, btn_size) {
                    self.append_char('3');
                }
                if self.calc_btn(ui, "-", GOLD_ACCENT, btn_size) {
                    self.append_char('-');
                }
                ui.end_row();

                if self.calc_btn(ui, "0", CARD_BG, btn_size) {
                    self.append_char('0');
                }
                if self.calc_btn(ui, ".", CARD_BG, btn_size) {
                    self.append_char('.');
                }

                if self.calc_btn(ui, "=", SUCCESS_GREEN, btn_size) {
                    self.calculate_expression();
                }
                if self.calc_btn(ui, "+", GOLD_ACCENT, btn_size) {
                    self.append_char('+');
                }
                ui.end_row();
            });

        ui.add_space(20.0);

        ui.label(RichText::new("Histórico").strong().color(GOLD_ACCENT));
        ui.separator();
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for (expr, res) in self.history.iter().rev() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(expr).color(Color32::GRAY));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("= {}", res))
                                    .strong()
                                    .color(TEXT_WHITE),
                            );
                        });
                    });
                    ui.separator();
                }
            });
    }

    fn render_compound(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Valor Inicial (R$)", &mut self.input_a);
        Self::draw_input(ui, "Aporte Mensal (R$)", &mut self.input_b);
        Self::draw_input(ui, "Taxa de Juros Mensal (%)", &mut self.input_c);
        Self::draw_input(ui, "Tempo (Meses)", &mut self.input_d);
    }

    fn render_simple(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Capital Inicial (R$)", &mut self.input_a);
        Self::draw_input(ui, "Taxa de Juros Mensal (%)", &mut self.input_b);
        Self::draw_input(ui, "Tempo (Meses)", &mut self.input_c);
    }

    fn render_magic_number(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Preço da Cota (R$)", &mut self.input_a);
        Self::draw_input(ui, "Último Dividendo Pago (R$)", &mut self.input_b);
    }

    fn render_fuel_compare(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Preço Etanol (R$)", &mut self.input_a);
        Self::draw_input(ui, "Preço Gasolina (R$)", &mut self.input_b);
        Self::draw_input(ui, "Consumo na Gasolina (Km/L)", &mut self.input_c);
        Self::draw_input(ui, "Quanto quer abastecer? (R$)", &mut self.input_d);
    }

    fn render_fuel_efficiency(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Valor Total Abastecido (R$)", &mut self.input_a);
        Self::draw_input(ui, "Preço do Litro (R$)", &mut self.input_b);
        Self::draw_input(ui, "Km Percorridos", &mut self.input_c);
    }

    fn render_ir(&mut self, ui: &mut egui::Ui) {
        Self::draw_input(ui, "Salário Bruto (R$)", &mut self.input_a);
        Self::draw_input(ui, "Número de Dependentes", &mut self.input_b);
        Self::draw_input(ui, "Outros Descontos (R$)", &mut self.input_c);
    }

    fn draw_input(ui: &mut egui::Ui, label: &str, value: &mut String) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).strong().color(TEXT_WHITE));
        });

        ui.add(
            egui::TextEdit::singleline(value)
                .desired_width(f32::INFINITY)
                .margin(Vec2::splat(4.0)),
        );
        ui.add_space(10.0);
    }

    fn get_mode_name(&self) -> String {
        match self.mode {
            CalculatorMode::Basic => "Operações Básicas".into(),
            CalculatorMode::CompoundInterest => "Juros Compostos".into(),
            CalculatorMode::SimpleInterest => "Juros Simples".into(),
            CalculatorMode::MagicNumber => "Número Mágico".into(),
            CalculatorMode::FuelCompare => "Combustível: Qual abastecer?".into(),
            CalculatorMode::FuelEfficiency => "Combustível: Média de Consumo".into(),
            CalculatorMode::IncomeTax => "Salário Líquido (IRRF)".into(),
        }
    }

    fn get_description_title(&self) -> String {
        match self.mode {
            CalculatorMode::Basic => "Calculadora Padrão".into(),
            CalculatorMode::CompoundInterest => "O Poder do Tempo".into(),
            CalculatorMode::SimpleInterest => "Rendimento Linear".into(),
            CalculatorMode::MagicNumber => "Efeito Bola de Neve".into(),
            CalculatorMode::FuelCompare => "Etanol ou Gasolina?".into(),
            CalculatorMode::FuelEfficiency => "Qual o consumo real?".into(),
            CalculatorMode::IncomeTax => "Quanto sobra?".into(),
        }
    }

    fn get_description_text(&self) -> String {
        match self.mode {
            CalculatorMode::Basic => "Realize somas, subtrações, multiplicações e divisões rápidas.".into(),
            CalculatorMode::CompoundInterest => "Descubra quanto seu dinheiro rende com juros sobre juros. Ideal para planejar aposentadoria e investimentos de longo prazo.".into(),
            CalculatorMode::SimpleInterest => "Cálculo de rendimento onde os juros incidem apenas sobre o valor principal. Comum em empréstimos entre pessoas ou títulos específicos.".into(),
            CalculatorMode::MagicNumber => "Descubra quantas cotas de um Fundo Imobiliário ou Ação você precisa ter para comprar uma nova cota apenas com os dividendos recebidos.".into(),
            CalculatorMode::FuelCompare => "Compare os preços e descubra quantos KM você poderá rodar com um valor específico em R$.".into(),
            CalculatorMode::FuelEfficiency => "Descubra quantos Km/L seu carro fez baseado no valor abastecido e na distância percorrida.".into(),CalculatorMode::IncomeTax => "Estime seu salário líquido descontando INSS e Imposto de Renda base (Tabela 2024/2025). Cálculo simplificado.".into(),
        }
    }

    fn calculate(&mut self) {
        let parse = |s: &str| s.replace(',', ".").parse::<f64>().unwrap_or(0.0);
        let a = parse(&self.input_a);
        let b = parse(&self.input_b);
        let c = parse(&self.input_c);
        let d = parse(&self.input_d);

        match self.mode {
            CalculatorMode::Basic => {
                let op = self.input_c.trim();
                let res = match op {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b != 0.0 {
                            a / b
                        } else {
                            0.0
                        }
                    }
                    _ => 0.0,
                };
                self.result_text = format!("{:.2}", res);
                self.detail_text = String::new();
            }
            CalculatorMode::CompoundInterest => {
                let p = a;
                let pm = b;
                let r = c / 100.0;
                let t = d as i32;

                let mut total = p;
                for _ in 0..t {
                    total = total * (1.0 + r) + pm;
                }
                let total_investido = p + (pm * t as f64);
                let juros = total - total_investido;

                self.result_text = format!("R$ {:.2}", total);
                self.detail_text = format!(
                    "Total Investido: R$ {:.2}\nTotal em Juros: R$ {:.2}",
                    total_investido, juros
                );
            }
            CalculatorMode::SimpleInterest => {
                let p = a;
                let r = b / 100.0;
                let t = c;
                let juros = p * r * t;
                let total = p + juros;

                self.result_text = format!("R$ {:.2}", total);
                self.detail_text = format!("Juros acumulados: R$ {:.2}", juros);
            }
            CalculatorMode::MagicNumber => {
                let price = a;
                let div = b;
                if div > 0.0 {
                    let magic = (price / div).ceil();
                    let cost = magic * price;
                    self.result_text = format!("{:.0} Cotas", magic);
                    self.detail_text = format!("Custo estimado para atingir: R$ {:.2}", cost);
                } else {
                    self.result_text = "Erro".into();
                    self.detail_text = "Dividendo deve ser maior que zero.".into();
                }
            }
            CalculatorMode::FuelCompare => {
                let p_etanol = a;
                let p_gasolina = b;
                let consumo_gas = c;
                let budget = d;

                if p_gasolina > 0.0 && consumo_gas > 0.0 {
                    let ratio = p_etanol / p_gasolina;
                    let (vencedor, p_vencedor, consumo_vencedor) = if ratio <= 0.70 {
                        ("ETANOL", p_etanol, consumo_gas * 0.70)
                    } else {
                        ("GASOLINA", p_gasolina, consumo_gas)
                    };

                    let litros = budget / p_vencedor;
                    let autonomia = litros * consumo_vencedor;

                    self.result_text = format!("Abasteça com {}", vencedor);
                    self.detail_text = format!(
                        "Preço Relativo: {:.0}% da Gasolina.\n\nCom R$ {:.2} você coloca {:.2} litros.\nEstimativa de rodagem: {:.1} Km.",
                        ratio * 100.0, budget, litros, autonomia
                    );
                } else {
                    self.result_text = "Dados inválidos".into();
                    self.detail_text = "Preencha os preços e consumo.".into();
                }
            }

            CalculatorMode::FuelEfficiency => {
                let total_pago = a;
                let preco_litro = b;
                let distancia = c;

                if preco_litro > 0.0 && total_pago > 0.0 {
                    let litros = total_pago / preco_litro;
                    let media = distancia / litros;
                    let custo_km = total_pago / distancia;

                    self.result_text = format!("{:.2} Km/L", media);
                    self.detail_text = format!(
                        "Você abasteceu {:.2} litros.\nCusto por Km rodado: R$ {:.2}",
                        litros, custo_km
                    );
                } else {
                    self.result_text = "Dados inválidos".into();
                    self.detail_text = String::new();
                }
            }
            CalculatorMode::IncomeTax => {
                let bruto = a;
                let dependentes = b as i32;
                let descontos_extras = c;

                let inss = if bruto <= 1412.0 {
                    bruto * 0.075
                } else if bruto <= 2666.68 {
                    (1412.0 * 0.075) + ((bruto - 1412.0) * 0.09)
                } else if bruto <= 4000.03 {
                    (1412.0 * 0.075) + ((2666.68 - 1412.0) * 0.09) + ((bruto - 2666.68) * 0.12)
                } else if bruto <= 7786.02 {
                    (1412.0 * 0.075)
                        + ((2666.68 - 1412.0) * 0.09)
                        + ((4000.03 - 2666.68) * 0.12)
                        + ((bruto - 4000.03) * 0.14)
                } else {
                    908.85
                };

                let deducao_dep = dependentes as f64 * 189.59;
                let base_ir = bruto - inss - deducao_dep - descontos_extras;

                let irrf = if base_ir <= 2259.20 {
                    0.0
                } else if base_ir <= 2826.65 {
                    (base_ir * 0.075) - 169.44
                } else if base_ir <= 3751.05 {
                    (base_ir * 0.15) - 381.44
                } else if base_ir <= 4664.68 {
                    (base_ir * 0.225) - 662.77
                } else {
                    (base_ir * 0.275) - 896.00
                };

                let irrf_final = if irrf < 0.0 { 0.0 } else { irrf };
                let liquido = bruto - inss - irrf_final - descontos_extras;

                self.result_text = format!("R$ {:.2}", liquido);
                self.detail_text = format!(
                    "INSS: -R$ {:.2}\nIRRF: -R$ {:.2}\nDesc. Totais: -R$ {:.2}",
                    inss,
                    irrf_final,
                    inss + irrf_final + descontos_extras
                );
            }
        }
    }

    fn calc_btn(&mut self, ui: &mut egui::Ui, text: &str, color: Color32, size: Vec2) -> bool {
        let text_color = if color == GOLD_ACCENT || color == SUCCESS_GREEN {
            Color32::BLACK
        } else {
            TEXT_WHITE
        };
        let btn = egui::Button::new(RichText::new(text).size(20.0).strong().color(text_color))
            .fill(color)
            .min_size(size)
            .corner_radius(8.0);
        ui.add(btn).clicked()
    }

    fn append_char(&mut self, c: char) {
        self.expression.push(c);
    }

    fn handle_keyboard_input(&mut self, ui: &mut egui::Ui) {
        ui.input(|i| {
            for event in &i.events {
                if let egui::Event::Text(text) = event {
                    for c in text.chars() {
                        if "0123456789+-*/.()".contains(c) {
                            self.append_char(c);
                        } else if c == '=' {
                            self.calculate_expression();
                        }
                    }
                }
            }

            if i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Enter) {
                self.calculate_expression();
            }

            if i.key_pressed(egui::Key::Backspace) {
                self.expression.pop();
            }

            if i.key_pressed(egui::Key::Escape) {
                self.expression.clear();
                self.result_text = "0".into();
            }
        });
    }

    fn calculate_expression(&mut self) {
        if self.expression.is_empty() {
            return;
        }

        let input = self.expression.clone();

        let result = self.evaluate_simple_string(&input);

        match result {
            Ok(val) => {
                let res_str = format!("{:.2}", val)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string();
                self.history.push((input, res_str.clone()));
                self.expression = res_str;
            }
            Err(_) => {
                self.expression = "Erro".to_string();
            }
        }
    }

    fn evaluate_simple_string(&self, expr: &str) -> Result<f64, String> {
        meval::eval_str(expr).map_err(|_| "Erro".to_string())
    }
}
