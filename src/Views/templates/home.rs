use super::*;
use crate::{
    control::{recover, recover_years},
    model::List::get_counts_instance,
};

#[path = "./boxes/to_home/mod.rs"]
mod to_home;
use chrono::Datelike;
use control::get_user_instance;
use to_home::*;

pub fn home_screen(screen_master: &Stack) -> Box {
    let screen = Box::new(Orientation::Horizontal, 0);

    let stack = Stack::new();

    let run = tokio::runtime::Runtime::new().unwrap();
    if let Ok(years) = run.block_on(recover_years()) {
        if !years.is_empty() {
            run.block_on(recover(years[0])).unwrap();
        } else {
            let _ = run
                .block_on(recover(crate::chrono::Utc::now().year() as i16))
                .map_err(|err| println!("{}", err));
        }
    }

    let today = chrono::Utc::now();

    if today.month()
        != get_user_instance()
            .as_ref()
            .unwrap()
            .last_login
            .parse::<u32>()
            .unwrap()
    {
        if today.month() == 1 {
            alert("Pronto para começar mais um ano? Tenho certeza que neste as coisas serão ainda melhores!", "Feliz ano novo!")
        } else {
            let month_perfomance = get_counts_instance().get_perfomance_months();
            let month_perfomance = month_perfomance[(today.month() - 2) as usize];

            if month_perfomance < 0.0 {
                alert(&format!("Que triste :´( Seu último mês teve um rendimento negativo de R${month_perfomance:.2}"), "Abre o olho!");
            } else if month_perfomance > 0.0 {
                alert(
                    &format!(
                        "Parabéns!!! Seu último mês teve um rendimento positivo de R${month_perfomance:.2}"
                    ),
                    "Você positivou!",
                );
            } else {
                alert(
                    "Seu último mês teve rendimento de R$0,00",
                    "Nem pra mais, nem pra menos!",
                );
            }
        }
    }

    let box_menu_left = get_box_menu_left(&stack);
    let box_body = get_box_body(&stack, &screen_master);

    screen.set_valign(gtk::Align::Center);
    screen.set_halign(gtk::Align::Center);

    screen.append(&box_menu_left);
    screen.append(&box_body);

    screen
}
