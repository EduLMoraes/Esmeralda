use super::{DebtError, DebtService};
use chrono::{Datelike, NaiveDate};
use esmeralda_entities::debt::{Debt, NatureDebt};

pub struct DebtServiceImpl;

impl DebtService for DebtServiceImpl {
    fn pay_installment(&self, debt: &mut Debt) -> Result<(), DebtError> {
        if debt.paid_installments < debt.installments {
            debt.paid_installments += 1;
            if debt.paid_installments == debt.installments {
                debt.status = true;
            }
            Ok(())
        } else {
            Err(DebtError::ErrorToPay)
        }
    }

    fn pay_all(&self, debt: &mut Debt) -> Result<(), DebtError> {
        debt.paid_installments = debt.installments;
        debt.status = true;
        Ok(())
    }

    fn calculate_end_date(&self, debt: &Debt) -> NaiveDate {
        if let NatureDebt::Incoming = debt.nature {
            return debt.date_start;
        }

        let mut tmp_month = debt.date_start.month() + debt.installments as u32;
        let mut tmp_year = debt.date_start.year();
        let mut tmp_installments = tmp_month;

        while tmp_month > 12 {
            tmp_year += 1;
            tmp_installments -= 12;
            tmp_month = tmp_installments;
        }

        NaiveDate::from_ymd_opt(tmp_year, tmp_month, 1).unwrap_or(debt.date_start)
    }
}
