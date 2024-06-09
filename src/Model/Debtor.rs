#[derive(Debug, Clone)]
pub struct Debtor {
    id: i32,
    name: String,
    debt: f32,
    value: f32,
    status: bool,
}

impl Debtor {
    pub fn new(id: i32, name: &str, debt: f32, value: f32) -> Debtor {
        let stt = debt <= value;

        Debtor {
            id,
            name: name.to_string(),
            debt,
            value,
            status: stt,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_debt(&self) -> f32 {
        self.debt
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_status(&self) -> bool {
        self.status
    }

    pub fn add_value(&mut self, v: f32) {
        self.value += v;

        if self.debt == 0.0 {
            self.status = true;
        }
    }

    pub fn add_debt(&mut self, d: f32) {
        self.debt += d;

        if self.debt > 0.0 {
            self.status = false;
        }
    }
}
