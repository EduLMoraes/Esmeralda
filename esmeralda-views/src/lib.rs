use eframe::egui;
use egui::Color32;

use crate::{
    home::HomeScreen,
    login::{LoginAction, LoginScreen},
    register::{RegisterAction, RegisterScreen},
};
pub mod add_account;
pub mod home;
pub mod login;
pub mod register;

const ICON_SIZE: f32 = 32.0;
const GOLD_ACCENT: Color32 = Color32::from_rgb(212, 175, 55);
const SLATE_BG: Color32 = Color32::from_rgb(26, 28, 30);
const CARD_BG: Color32 = Color32::from_rgb(35, 37, 40);
const SUCCESS_GREEN: Color32 = Color32::from_rgb(76, 175, 80);
const DEBT_RED: Color32 = Color32::from_rgb(155, 0, 0);

pub enum AppState {
    Login,
    Register,
    Dashboard,
}
pub struct EsmeraldaApp {
    pub state: AppState,
    pub login_screen: LoginScreen,
    pub register_screen: RegisterScreen,
    pub home_screen: HomeScreen, // Adicionado
}

impl eframe::App for EsmeraldaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_custom_style(ctx);

        match self.state {
            AppState::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(action) = self.login_screen.ui(ui) {
                        match action {
                            LoginAction::Submit => self.state = AppState::Dashboard, // Vai para Home
                            LoginAction::GoToRegister => self.state = AppState::Register,
                        }
                    }
                });
            }
            AppState::Register => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(action) = self.register_screen.ui(ui) {
                        match action {
                            RegisterAction::Submit => self.state = AppState::Dashboard,
                            RegisterAction::GoToLogin => self.state = AppState::Login,
                        }
                    }
                });
            }
            AppState::Dashboard => {
                // A Home gerencia seus próprios painéis (Side/Central)
                self.home_screen.ui(ctx);
            }
        }
    }
}

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // Tons de cinza neutros para a base
    visuals.panel_fill = egui::Color32::from_rgb(18, 18, 18); // Fundo quase preto
    visuals.window_fill = egui::Color32::from_rgb(26, 28, 30); // Cards e janelas

    // Ouro para a marca e interações
    let gold = egui::Color32::from_rgb(212, 175, 55);
    visuals.selection.bg_fill = gold.gamma_multiply(0.3);
    visuals.widgets.active.bg_fill = gold;
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(184, 134, 11);

    ctx.set_visuals(visuals);
}
