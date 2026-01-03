use crate::home::HomeScreen;
use crate::login::{LoginAction, LoginScreen};
use crate::register::{RegisterAction, RegisterScreen};
use eframe::egui;
use esmeralda_services::UserService;
use esmeralda_debt::DebtService;
use std::sync::Arc;

pub mod add_debt;
pub mod add_incoming;
pub mod calculator;
pub mod dashboard;
pub mod home;
pub mod login;
pub mod register;

pub const PADDING: f32 = 12.0;
pub const GOLD_ACCENT: egui::Color32 = egui::Color32::from_rgb(222, 164, 55);
pub const DEBT_RED: egui::Color32 = egui::Color32::from_rgb(255, 105, 97);
pub const SUCCESS_GREEN: egui::Color32 = egui::Color32::from_rgb(144, 238, 144);
pub const SLATE_BG: egui::Color32 = egui::Color32::from_rgb(31, 31, 46);
pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(41, 41, 64);
pub const ICON_SIZE: f32 = 24.0;
pub const TEXT_WHITE: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);

pub enum AppState {
    Login,
    Register,
    Dashboard,
}
pub struct EsmeraldaApp {
    pub state: AppState,
    pub login_screen: LoginScreen,
    pub register_screen: RegisterScreen,
    pub home_screen: HomeScreen,
}

impl EsmeraldaApp {
    pub fn new(user_service: Arc<dyn UserService>, debt_service: Arc<dyn DebtService>) -> Self {
        let login_screen = LoginScreen::new(user_service.clone());
        let register_screen = RegisterScreen::new(user_service.clone());
        let home_screen = HomeScreen::new(user_service, debt_service);

        Self {
            state: AppState::Login,
            login_screen,
            register_screen,
            home_screen,
        }
    }
}

impl eframe::App for EsmeraldaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_custom_style(ctx);

        match self.state {
            AppState::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(action) = self.login_screen.ui(ui) {
                        match action {
                            LoginAction::Submit(user) => {
                                self.home_screen.set_user(user);
                                self.state = AppState::Dashboard;
                            },
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
                self.home_screen.ui(ctx);
            }
        }
    }
}

pub fn apply_custom_style(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = egui::Color32::from_rgb(18, 18, 18);
    visuals.window_fill = egui::Color32::from_rgb(26, 28, 30);

    let gold = egui::Color32::from_rgb(212, 175, 55);
    visuals.selection.bg_fill = gold.gamma_multiply(0.3);
    visuals.widgets.active.bg_fill = gold;
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(184, 134, 11);

    ctx.set_visuals(visuals);
}