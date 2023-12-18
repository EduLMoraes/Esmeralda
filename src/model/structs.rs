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
    pub paid_installments: String,
    pub installments: String,
    pub status: bool
}

#[derive(Clone, Debug)]
pub struct InterfaceInfo{
    list: Vec<Info>
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
        self.list.push(value)
    }

    pub fn get(&self, index: usize) -> &Info{
        &self.list[index]
    }

    pub fn order_by_id(&mut self, crescent: bool){
        let width: usize = self.list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if self.list[i].id < self.list[i - 1].id{
                        comparisions = false;

                        let tmp = self.list[i].clone();

                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if self.list[i].id > self.list[i - 1].id{
                        comparisions = false;

                        let tmp = self.list[i].clone();

                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
    }

    pub fn order_by_date_in(&mut self, crescent: bool){
        let width: usize = self.list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent{
                for i in 1..width{ 
                    
                    if self.list[i].date_in < self.list[i - 1].date_in{
                        comparisions = false;
    
                        let tmp = self.list[i].clone();
    
                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                    
                    if self.list[i].date_in > self.list[i - 1].date_in{
                        comparisions = false;
    
                        let tmp = self.list[i].clone();
    
                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
    }

    pub fn order_by_date_out(&mut self, crescent: bool){
        let width: usize = self.list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent{
                for i in 1..width{ 
                    
                    if self.list[i].date_out < self.list[i - 1].date_out{
                        comparisions = false;
    
                        let tmp = self.list[i].clone();
    
                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                    
                    if self.list[i].date_out > self.list[i - 1].date_out{
                        comparisions = false;
    
                        let tmp = self.list[i].clone();
    
                        self.list[i] = self.list[i - 1].clone();
                        self.list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
    }

    pub fn order_alphabetical(&mut self){
        let width: usize = self.list.len();

        loop{
            let mut comparisions: bool = true;

            for i in 1..width{ 
                        
                if self.list[i].debtor > self.list[i - 1].debtor{
                    comparisions = false;
    
                    let tmp = self.list[i].clone();
    
                    self.list[i] = self.list[i - 1].clone();
                    self.list[i - 1] = tmp;
                }
            }

            if comparisions{
                break;
            }
        }
    }

    pub fn test() -> InterfaceInfo{

        let mut vector: Vec<Info> = Vec::new();

        let names = vec!["Eduardo", "Ana Luiza", "Eduardo Almeida", "Huan Zerton", "Huan Alter"];

        for _ in 0..4{
            let d = thread_rng().gen_range(1..31);
            let m = thread_rng().gen_range(1..12);
            let y = thread_rng().gen_range(1900..2023);

            let date_in = NaiveDate::from_ymd_opt(y, m, d).unwrap();

            let d = thread_rng().gen_range(1..31);
            let m = thread_rng().gen_range(1..12);
            let y = thread_rng().gen_range(1900..2023);
            let date_out = NaiveDate::from_ymd_opt(y, m, d).unwrap();

            let info_test = Info{
                id: rand::thread_rng().gen_range(0..9000),
                debtor: names[rand::thread_rng().gen_range(0..names.len())].to_string(),
                title: "Grão de Arroz".to_string(),
                description: "Ele comprou grão por grão".to_string(),
                value: rand::thread_rng().gen_range(0.0..102.78),
                date_in: date_in,
                date_out: date_out,
                paid_installments: "5".to_string(),
                installments: "12".to_string(),
                status: rand::thread_rng().gen_bool(0.5)
            };
            
            vector.push(info_test.clone());
        }

        InterfaceInfo{
            list: vector
        }
    }
}