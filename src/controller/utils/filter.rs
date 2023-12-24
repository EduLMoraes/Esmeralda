use super::*;
use std::collections::HashMap;

#[allow(unreachable_code)]
/// Filters the list of debt information for each debtor and calculates the total value and debt for each debtor.
///
/// # Arguments
///
/// * `list` - A vector of `Info` structs representing the list of debt information for each debtor.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
///
/// #[derive(Debug)]
/// struct Info {
///     id: u32,
///     debtor: &'static str,
///     value: f64,
///     status: bool,
/// }
///
/// #[derive(Debug)]
/// struct Debtor {
///     id: u32,
///     name: String,
///     total_value: f64,
///     total_debt: f64,
/// }
///
/// impl Debtor {
///     fn new(id: u32, name: String, total_value: f64, total_debt: f64) -> Self {
///         Debtor {
///             id,
///             name,
///             total_value,
///             total_debt,
///         }
///     }
///
///     fn add_value(&mut self, value: f64) {
///         self.total_value += value;
///     }
///
///     fn add_debt(&mut self, debt: f64) {
///         self.total_debt += debt;
///     }
///
///     fn get_id(&self) -> u32 {
///         self.id
///     }
///
///     fn get_name(&self) -> &str {
///         &self.name
///     }
///
///     fn get_total_value(&self) -> f64 {
///         self.total_value
///     }
///
///     fn get_total_debt(&self) -> f64 {
///         self.total_debt
///     }
/// }
///
/// #[allow(unreachable_code)]
/// pub fn filter_debtors(list: Vec<Info>) -> Vec<Debtor> {
///     let mut debtors_map: HashMap<String, Debtor> = HashMap::new();
///
///     for info in list {
///         let name = info.debtor.trim().to_string();
///         let debtor = debtors_map.entry(name.clone()).or_insert(Debtor::new(info.id, name.clone().trim(), 0.0, 0.0));
///
///         if info.status {
///             debtor.add_value(info.value);
///         } else {
///             debtor.add_value(info.value);
///             debtor.add_debt(info.value);
///         }
///     }
///
///     let mut debtors: Vec<Debtor> = debtors_map.into_values().collect();
///     debtors.sort_by_key(|debtor| debtor.get_id());
///
///     debtors
/// }
/// ```
pub fn filter_debtors(list: Vec<Info>) -> Vec<Debtor> {
    let mut debtors_map: HashMap<String, Debtor> = HashMap::new();
    
    for info in list {
        let name = info.debtor.trim().to_string();
        let debtor = debtors_map.entry(name.clone()).or_insert(Debtor::new(info.id, name.clone().trim(), 0.0, 0.0));
        
        if info.status {
            debtor.add_value(info.value);
        } else {
            debtor.add_value(info.value);
            debtor.add_debt(info.value);
        }
    }

    let mut debtors: Vec<Debtor> = debtors_map.into_values().collect();
    debtors.sort_by_key(|debtor| debtor.get_id()); // Ordenar se necessário por ID

    debtors
}
