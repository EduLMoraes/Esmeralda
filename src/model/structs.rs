use crate::chrono::NaiveDate;
use crate::control::get_user_instance;

/// The `Info` struct represents information about a debtor.
///
/// # Fields
///
/// - `id`: An integer representing the ID of the debtor.
/// - `debtor`: A string representing the name of the debtor.
/// - `title`: A string representing the title of the debtor.
/// - `description`: A string representing the description of the debtor.
/// - `value`: A float representing the value of the debtor.
/// - `date_in`: A `NaiveDate` representing the date the debtor was created.
/// - `date_out`: A `NaiveDate` representing the date the debtor is due.
/// - `paid_installments`: An unsigned integer representing the number of paid installments.
/// - `installments`: An unsigned integer representing the total number of installments.
/// - `status`: A boolean representing the status of the debtor.
///
/// # Example Usage
///
/// ```rust
/// let mut info = Info::new();
/// info.new_id();
/// ```
#[derive(Clone, Debug)]
pub struct Info {
    pub id: i32,
    pub debtor: String,
    pub title: String,
    pub description: String,
    pub value: f32,
    pub date_in: NaiveDate,
    pub date_out: NaiveDate,
    pub paid_installments: u32,
    pub installments: u32,
    pub status: bool,
}

impl Info {
    /// Creates a new instance of the `Info` struct.
    ///
    /// The `id` field is initialized by concatenating the user's ID with a '0' and parsing it as an integer.
    /// The other fields are initialized with default values.
    ///
    /// # Returns
    ///
    /// An instance of the `Info` struct with initialized fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// let info = Info::new();
    /// ```
    pub fn new() -> Info {
        let today = chrono::Utc::now();
        let user = get_user_instance().as_ref().unwrap().clone();

        Info {
            id: format!("{}0{}", user.id, 0).trim().parse::<i32>().unwrap(),
            debtor: String::new(),
            title: String::new(),
            description: String::new(),
            value: 0.0,
            date_in: today.date_naive(),
            date_out: today.date_naive(),
            paid_installments: 1,
            installments: 1,
            status: false,
        }
    }

    /// Generates a new ID for the debtor.
    ///
    /// The `id` field is incremented by 1.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut info = Info::new();
    /// info.new_id();
    /// ```
    pub fn new_id(&mut self) {
        let id = self.id + 1;
        self.id = id;
    }
}
/// Represents a collection of `Info` objects and provides methods to manipulate and order the list based on different criteria.
///
/// Example Usage:
/// ```
/// let mut interface = InterfaceInfo::new();
///
/// let info1 = Info { id: 1, debtor: "John", value: 100.0 };
/// let info2 = Info { id: 2, debtor: "Alice", value: 200.0 };
///
/// interface.put(info1);
/// interface.put(info2);
///
/// let ordered_list = interface.order_by_id(true);
///
/// println!("{}", ordered_list);
/// ```
///
/// Outputs:
/// ```
/// Info { id: 1, debtor: "John", value: 100.0 }
/// Info { id: 2, debtor: "Alice", value: 200.0 }
/// ```
///
/// Inputs:
/// - `value`: A `Info` object to be inserted into the list.
/// - `crescent`: A boolean value indicating whether the list should be ordered in ascending or descending order.
/// - `column`: A string indicating the column to be used for alphabetical ordering.
///
/// Outputs:
/// - The `len` method returns the number of `Info` objects in the list.
/// - The `put` method inserts a `Info` object into the list.
/// - The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods return a new `InterfaceInfo` object with the ordered list.
/// - The `order_alphabetical` method returns a new `InterfaceInfo` object with the alphabetically ordered list.
/// - The `test` method returns a randomly generated `InterfaceInfo` object for testing purposes.
/// - The `Display` trait implementation allows printing the list of `Info` objects.
///
/// Flow:
/// 1. The `InterfaceInfo` struct has a `list` field that stores a vector of `Info` objects.
/// 2. The `new` method initializes an empty `InterfaceInfo` object.
/// 3. The `len` method returns the length of the list.
/// 4. The `put` method inserts a `Info` object at the beginning of the list.
/// 5. The `order_by_id`, `order_by_value`, `order_by_status`, `order_by_date`, and `order_by_installments` methods order the list based on the specified criteria.
/// 6. The `order_alphabetical` method orders the list alphabetically based on the specified column.
/// 7. The `test` method generates a random list of `Info` objects for testing purposes.
/// 8. The `Display` trait implementation allows printing the list of `Info` objects.
#[derive(Clone, Debug)]
pub struct InterfaceInfo{
    pub list: Vec<Info>
}

use rand::{Rng, thread_rng};
impl InterfaceInfo{
    #[allow(dead_code)]
    pub fn new() -> InterfaceInfo{
        InterfaceInfo{ list: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn put(&mut self, value: Info) {
        self.list.insert(0, value)
    }

    pub fn order_by_id(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list.list[i].id < list.list[i - 1].id{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list.list[i].id > list.list[i - 1].id{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        list
    }

    pub fn order_by_value(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list.list[i].value < list.list[i - 1].value{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list.list[i].value > list.list[i - 1].value{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        list
    }

    pub fn order_by_status(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list.list[i].status < list.list[i - 1].status{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list.list[i].status > list.list[i - 1].status{
                        comparisions = false;

                        let tmp = list.list[i].clone();

                        list.list[i] = list.list[i - 1].clone();
                        list.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        list
    }

    pub fn order_by_date(&self, is_in: bool, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = self.list.len();

        if is_in{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].date_in < list.list[i - 1].date_in{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].date_in > list.list[i - 1].date_in{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }else{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].date_out < list.list[i - 1].date_out{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].date_out > list.list[i - 1].date_out{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }   
        list
    }

    pub fn order_by_installments(&self, is_paid: bool, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = self.list.len();

        if is_paid{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].paid_installments < list.list[i - 1].paid_installments{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].paid_installments > list.list[i - 1].paid_installments{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }else{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].installments < list.list[i - 1].installments{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].installments > list.list[i - 1].installments{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }   
        list
    }

    pub fn order_alphabetical(&self, column: &str, crescent: bool) -> InterfaceInfo{
        let mut list = self.clone();
        let width: usize = self.list.len();

        if column == "name" {
            loop{
                let mut comparisions: bool = true;
    
                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].debtor < list.list[i - 1].debtor{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].debtor > list.list[i - 1].debtor{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }
    
                if comparisions{
                    break;
                }
            }
        }else if column == "title" {
            loop{
                let mut comparisions: bool = true;
    
                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].title < list.list[i - 1].title{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].title > list.list[i - 1].title{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }
    
                if comparisions{
                    break;
                }
            }
        }else if column == "description" {
            loop{
                let mut comparisions: bool = true;
    
                if crescent{
                    for i in 1..width{ 
                        
                        if list.list[i].description < list.list[i - 1].description{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list.list[i].description > list.list[i - 1].description{
                            comparisions = false;
        
                            let tmp = list.list[i].clone();
        
                            list.list[i] = list.list[i - 1].clone();
                            list.list[i - 1] = tmp;
                        }
                    }
                }
    
                if comparisions{
                    break;
                }
            }
        }
        list
    }

    #[allow(dead_code)]
    pub fn test() -> InterfaceInfo{

        let mut vector: Vec<Info> = Vec::new();

        let names = vec!["Eduardo", "Ana Luiza", "Eduardo Almeida", "Huan Zerton", "Huan Alter"];

        for _ in 0..10{
            let d = thread_rng().gen_range(1..29);
            let m = thread_rng().gen_range(1..12);
            let y = thread_rng().gen_range(1900..2023);

            let date_in = NaiveDate::from_ymd_opt(y, m, d).unwrap();

            let d = thread_rng().gen_range(1..29);
            let m = thread_rng().gen_range(1..12);
            let y = thread_rng().gen_range(1900..2023);
            let date_out = NaiveDate::from_ymd_opt(y, m, d).unwrap();

            let info_test = Info{
                id: rand::thread_rng().gen_range(1000..9999),
                debtor: names[rand::thread_rng().gen_range(0..names.len())].to_string(),
                title: "Grão de Arroz".to_string(),
                description: "Ele comprou grão por grão".to_string(),
                value: rand::thread_rng().gen_range(0.0..1002.78),
                date_in: date_in,
                date_out: date_out,
                paid_installments: 5,
                installments: 12,
                status: rand::thread_rng().gen_bool(0.5)
            };
            
            vector.push(info_test.clone());
        }

        InterfaceInfo{
            list: vector
        }
    }
}

use std::fmt;
impl fmt::Display for InterfaceInfo{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len(){
            write!(f, "{:?}\n", self.list[i])?;
        }

        Ok(())
    }
}
#[derive(Debug, Clone)]
/// Represents a debtor with an ID, name, debt amount, value amount, and status.
pub struct Debtor{
    id: i32,
    name: String,
    debt: f32,
    value: f32,
    status: bool
}

impl Debtor{
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
    pub fn new(id: i32, name: &str, debt: f32, value: f32) -> Debtor{
        let stt = debt <= value;

        Debtor { id: id, name: name.to_string(), debt: debt, value: value, status:  stt}
    }

    /// Returns a reference to the name of the debtor.
    pub fn get_name(&self) -> &String{
        &self.name
    }

    /// Returns the ID of the debtor.
    pub fn get_id(&self) -> i32{
        self.id
    }

    /// Returns the amount of debt owed by the debtor.
    pub fn get_debt(&self) -> f32{
        self.debt
    }

    /// Returns the value of assets owned by the debtor.
    pub fn get_value(&self) -> f32{
        self.value
    }

    /// Returns the status of the debtor (true if the debt is less than or equal to the value, false otherwise).
    pub fn get_status(&self) -> bool{
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
    pub fn add_value(&mut self, v: f32){
        self.value += v;

        if self.debt == 0.0{
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
    pub fn add_debt(&mut self, d: f32){
        self.debt += d;

        if self.debt > 0.0 {
            self.status = false;
        }
    }
}