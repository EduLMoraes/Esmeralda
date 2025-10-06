use crate::model::Count::Count;
use crate::model::List::ListCount;

pub async fn read_csv(path: &str, data: &mut ListCount) -> Result<(), String> {
    let mut reader = csv::Reader::from_path(path).map_err(|e| e.to_string())?;

    for record in reader.records() {
        let record = record.map_err(|e| format!("{e} to parse record"))?;
        if record[0] == "id".to_string() {
            continue;
        }
        for row in &record {
            let row = row.split(';').collect::<Vec<&str>>();
            let id = row[0]
                .parse::<i32>()
                .map_err(|e| format!("{e} to parse id"))?;
            let debtor = row[1].to_string();
            let nature = row[2].to_string();
            let title = row[3].to_string();
            let description = row[4].to_string();
            let date_in = row[5]
                .parse::<chrono::NaiveDate>()
                .map_err(|e| format!("{e} to parse date_in"))?;
            let date_out = row[6]
                .parse::<chrono::NaiveDate>()
                .map_err(|e| format!("{e} to parse date_out"))?;
            let paid_installments = row[7]
                .parse::<u32>()
                .map_err(|e| format!("{e} to parse paid_installments"))?;
            let installments = row[8]
                .parse::<u32>()
                .map_err(|e| format!("{e} to parse installments"))?;
            let value = row[9]
                .parse::<f32>()
                .map_err(|e| format!("{e} to parse value"))?;
            let status = row[10].to_string() == "Paga";
            let count = Count {
                id,
                debtor,
                nature,
                title,
                description,
                date_in,
                date_out,
                paid_installments,
                installments,
                value,
                status,
            };
            if !data.list.contains(&count) {
                data.list.push(count);
            }
        }
    }
    Ok(())
}
