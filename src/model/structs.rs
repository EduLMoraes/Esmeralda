use crate::NaiveDate;

pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone)]
pub struct User{
    pub username: String,
    pub password: String
}

#[derive(Clone, Debug)]
pub struct Info{
    pub id: i32,
    pub debtor: String,
    pub title: String,
    pub description: String,
    pub value: f64,
    pub date_in: NaiveDate,
    pub date_out: NaiveDate,
    pub paid_installments: u32,
    pub installments: u32,
    pub status: bool
}

impl Info{
    pub fn new() -> Info{
        let today = chrono::Utc::now();

        Info{
            id: 0,
            debtor: String::new(),
            title: String::new(),
            description: String::new(),
            value: 0.0,
            date_in: today.date_naive(),
            date_out: today.date_naive(),
            paid_installments: 1,
            installments: 1,
            status: false
        }
    }

    pub fn new_id(&mut self){
        self.id += 1;
    }
}

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

        for _ in 0..250{
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