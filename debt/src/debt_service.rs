use super::{DebtError, DebtService};
use chrono::{Datelike, NaiveDate};
use esmeralda_database::Database;
use esmeralda_entities::debt::{Debt, NatureDebt};
use std::sync::Arc;

pub struct DebtServiceImpl<D>
where
    D: Database<Debt>,
{
    debt_repo: Arc<D>,
}

impl<D> DebtServiceImpl<D>
where
    D: Database<Debt>,
{
    pub fn new(debt_repo: Arc<D>) -> Self {
        Self { debt_repo }
    }
}

impl<D> DebtService for DebtServiceImpl<D>
where
    D: Database<Debt>,
{
    fn get_all(&self, user_id: &str) -> Result<Vec<Debt>, DebtError> {
        self.debt_repo
            .get_all_from_user(user_id)
            .map_err(|_| DebtError::ErrorToPay)
    }

    fn insert(&self, debt: Debt) -> Result<(), DebtError> {
        self.debt_repo.insert(debt).map_err(|_| DebtError::ErrorToPay)
    }

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
