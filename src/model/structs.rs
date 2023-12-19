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
        self.list.push(value)
    }

    pub fn get(&self, index: usize) -> &Info{
        &self.list[index]
    }

    pub fn order_by_id(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list[i].id < list[i - 1].id{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list[i].id > list[i - 1].id{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        self.clone()
    }

    pub fn order_by_value(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list[i].value < list[i - 1].value{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list[i].value > list[i - 1].value{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        self.clone()
    }

    pub fn order_by_status(&self, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = list.len();

        loop{
            let mut comparisions: bool = true;

            if crescent {
                for i in 1..width{ 
                
                    if list[i].status < list[i - 1].status{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }else{
                for i in 1..width{ 
                
                    if list[i].status > list[i - 1].status{
                        comparisions = false;

                        let tmp = list[i].clone();

                        list[i] = list[i - 1].clone();
                        list[i - 1] = tmp;
                    }
                }
            }


            if comparisions{
                break;
            }
        }
        self.clone()
    }

    pub fn order_by_date(&self, is_in: bool, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = self.list.len();

        if is_in{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list[i].date_in < list[i - 1].date_in{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].date_in > list[i - 1].date_in{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
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
                        
                        if list[i].date_out < list[i - 1].date_out{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].date_out > list[i - 1].date_out{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }   

        self.clone()
    }

    pub fn order_by_installments(&self, is_paid: bool, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = self.list.len();

        if is_paid{
            loop{
                let mut comparisions: bool = true;

                if crescent{
                    for i in 1..width{ 
                        
                        if list[i].paid_installments < list[i - 1].paid_installments{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].paid_installments > list[i - 1].paid_installments{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
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
                        
                        if list[i].installments < list[i - 1].installments{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].installments > list[i - 1].installments{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }


                if comparisions{
                    break;
                }
            }
        }   

        self.clone()
    }

    pub fn order_alphabetical(&self, column: &str, crescent: bool) -> InterfaceInfo{
        let mut list = self.list.clone();
        let width: usize = self.list.len();

        if column == "name" {
            loop{
                let mut comparisions: bool = true;
    
                if crescent{
                    for i in 1..width{ 
                        
                        if list[i].debtor < list[i - 1].debtor{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].debtor > list[i - 1].debtor{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
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
                        
                        if list[i].title < list[i - 1].title{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].title > list[i - 1].title{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
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
                        
                        if list[i].description < list[i - 1].description{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }else{
                    for i in 1..width{ 
                        
                        if list[i].description > list[i - 1].description{
                            comparisions = false;
        
                            let tmp = list[i].clone();
        
                            list[i] = list[i - 1].clone();
                            list[i - 1] = tmp;
                        }
                    }
                }
    
                if comparisions{
                    break;
                }
            }
        }

        self.clone()
    }

    pub fn test() -> InterfaceInfo{

        let mut vector: Vec<Info> = Vec::new();

        let names = vec!["Eduardo", "Ana Luiza", "Eduardo Almeida", "Huan Zerton", "Huan Alter"];

        for _ in 0..23{
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