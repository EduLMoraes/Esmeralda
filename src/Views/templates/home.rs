use std::sync::Mutex;

use super::*;
use crate::{
    control::{recover, recover_years},
    model::List::get_counts_instance,
    prelude::model::Count::Count,
};

#[path = "./boxes/to_home/mod.rs"]
mod to_home;
use chrono::Datelike;
use control::get_user_instance;
use to_home::*;

const IS_ALERTED: Mutex<bool> = Mutex::new(false);

pub fn home_screen(screen_master: &Stack) -> Box {
    let screen = Box::new(Orientation::Horizontal, 0);

    let stack = Stack::new();

    let run = tokio::runtime::Runtime::new().unwrap();
    if let Ok(_years) = run.block_on(recover_years()) {
        let _ = run
            .block_on(recover(crate::chrono::Utc::now().year() as i16))
            .map_err(|err| println!("{}", err));
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

    #[allow(const_item_mutation)]
    if let Some(is_alerted) = IS_ALERTED.get_mut().ok() {
        if !*is_alerted {
            let counts = &get_counts_instance().list;
            let now = chrono::Utc::now().date_naive();
            let mut defeated_now: Vec<&Count> = Vec::new();
            let mut defeated_10days: Vec<&Count> = Vec::new();
            let mut defeated: Vec<&Count> = Vec::new();

            for count in counts {
                if !count.status {
                    let month_pay =
                        (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                    let months_to_pay = month_pay - now.month() as i32;
                    let days_to_pay = (count.date_out.day0() as i32 + (months_to_pay * 30) as i32)
                        - now.day0() as i32;
                    let years_to_pay = count.date_out.year() as i32 - now.year() as i32;
                    let is_late = months_to_pay <= 0 && days_to_pay < 0 && years_to_pay <= 0;

                    if !is_late {
                        if days_to_pay == 0 {
                            defeated_now.push(count);
                        } else if days_to_pay <= 10 {
                            defeated_10days.push(count);
                        }
                    } else {
                        defeated.push(count);
                    }
                }
            }

            if defeated_now.len() > 1 {
                alert(
                    &format!(
                        "{} contas estão vencendo hoje. Atente-se ao prazo!",
                        defeated_now.len()
                    ),
                    "Vencimento de contas",
                );
            } else if defeated_now.len() == 1 {
                alert(
                    &format!(
                        "A conta {} esta vencendo hoje. Atente-se ao prazo!",
                        defeated_now[0].title
                    ),
                    "Vencimento de contas",
                );
            }

            if defeated_10days.len() > 1 {
                alert(
                    &format!(
                        "{} contas estão próximas de vencer. Atente-se ao prazo!",
                        defeated_10days.len()
                    ),
                    "Vencimento de contas",
                );
            } else if defeated_10days.len() == 1 {
                let count = defeated[0];
                let month_pay = (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                let months_to_pay = month_pay - now.month() as i32;
                let days_to_pay = (count.date_out.day0() as i32 + (months_to_pay * 30) as i32)
                    - now.day0() as i32;

                alert(
                    &format!(
                        "Faltam {} dias para a conta {} vencer. Atente-se ao prazo!",
                        days_to_pay, count.title
                    ),
                    "Vencimento de contas",
                );
            }

            if defeated.len() > 1 {
                alert(
                    &format!(
                        "{} contas venceram! Atente-se ao prazo para não pagar juros!",
                        defeated.len()
                    ),
                    "Vencimento de contas",
                );
            } else if defeated.len() == 1 {
                let count = defeated[0];
                let month_pay = (count.date_in.month() + count.paid_installments + 1) as i32 % 12;
                let months_to_pay = month_pay - now.month() as i32;
                let days_to_pay = (count.date_out.day0() as i32 + (months_to_pay * 30) as i32)
                    - now.day0() as i32;

                alert(
                    &format!(
                        "A conta {} venceu há {}. Atente-se ao prazo!",
                        count.title, days_to_pay
                    ),
                    "Vencimento de contas",
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
