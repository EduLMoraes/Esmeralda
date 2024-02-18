#[derive(Debug, Clone)]
/// Represents a debtor with an ID, name, debt amount, value amount, and status.
pub struct Debtor {
    id: i32,
    name: String,
    debt: f32,
    value: f32,
    status: bool,
}

impl Debtor {
    /// Initializes a new `Debtor` with the provided ID, name, debt, and value.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the debtor.
    /// * `name` - The name of the debtor.
    /// * `debt` - The amount of debt owed by the debtor.
    /// * `value` - The value of assets owned by the debtor.
    ///
    /// # Example
    ///
    /// ```
    /// let debtor = Debtor::new(1, "John Doe", 100.0, 50.0);
    /// ```
    pub fn new(id: i32, name: &str, debt: f32, value: f32) -> Debtor {
        let stt = debt <= value;

        Debtor {
            id: id,
            name: name.to_string(),
            debt: debt,
            value: value,
            status: stt,
        }
    }

    /// Returns a reference to the name of the debtor.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the ID of the debtor.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Returns the amount of debt owed by the debtor.
    pub fn get_debt(&self) -> f32 {
        self.debt
    }

    /// Returns the value of assets owned by the debtor.
    pub fn get_value(&self) -> f32 {
        self.value
    }

    /// Returns the status of the debtor (true if the debt is less than or equal to the value, false otherwise).
    pub fn get_status(&self) -> bool {
        self.status
    }

    /// Updates the value amount of the debtor and potentially the status.
    ///
    /// # Arguments
    ///
    /// * `v` - The value to be added to the debtor's value amount.
    ///
    /// # Example
    ///
    /// ```
    /// debtor.add_value(25.0);
    /// ```
    pub fn add_value(&mut self, v: f32) {
        self.value += v;

        if self.debt == 0.0 {
            self.status = true;
        }
    }

    /// Updates the debt amount of the debtor and potentially the status.
    ///
    /// # Arguments
    ///
    /// * `d` - The debt to be added to the debtor's debt amount.
    ///
    /// # Example
    ///
    /// ```
    /// debtor.add_debt(75.0);
    /// ```
    pub fn add_debt(&mut self, d: f32) {
        self.debt += d;

        if self.debt > 0.0 {
            self.status = false;
        }
    }
}
