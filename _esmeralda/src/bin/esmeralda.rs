use eframe::NativeOptions;
use esmeralda_views::EsmeraldaApp;
use esmeralda_database::{Db, UserRepository, DebtRepository};
use esmeralda_cryptography::BcryptHasher;
use esmeralda_services::UserServiceImpl;
use esmeralda_debt::DebtServiceImpl;
use std::sync::Arc;
use esmeralda_apis::mailjet::MailjetApiImpl;

fn main() -> Result<(), eframe::Error> {
    let db = Db::new_with_path("esmeralda.db").unwrap();

    let user_repo = Arc::new(UserRepository::new(db.clone()));
    let debt_repo = Arc::new(DebtRepository::new(db.clone()));
    let mail_api = Arc::new(MailjetApiImpl);

    let hasher = Arc::new(BcryptHasher);
    let user_service = Arc::new(UserServiceImpl::new(user_repo, hasher, mail_api));
    let debt_service = Arc::new(DebtServiceImpl::new(debt_repo));

    let app = EsmeraldaApp::new(user_service, debt_service);

    let native_options = NativeOptions::default();

    eframe::run_native(
        "Esmeralda",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
