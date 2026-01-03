use eframe::NativeOptions;
use esmeralda_views::{
    home::HomeScreen, login::LoginScreen, register::RegisterScreen, AppState, EsmeraldaApp,
};

fn main() -> Result<(), eframe::Error> {
    let login_screen = LoginScreen::default();
    let register_screen = RegisterScreen::default();
    let home_screen = HomeScreen::default();

    let app = EsmeraldaApp {
        login_screen,
        register_screen,
        home_screen,
        state: AppState::Login,
    };

    let native_options = NativeOptions::default();

    eframe::run_native(
        "Esmeralda",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
