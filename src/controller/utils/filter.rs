use super::*;
use std::collections::HashMap;

#[allow(unreachable_code)]
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
