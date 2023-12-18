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
    pub date_in: String,
    pub date_out: String,
    pub paid_installments: String,
    pub installments: String,
    pub status: bool
}

use rand::Rng;
impl Info{
    pub fn test() -> Vec<Info>{

        let mut vector: Vec<Info> = Vec::new();

        for _ in 0..19{
            let info_test = Info{
                id: rand::thread_rng().gen_range(0..9000),
                debtor: "Guilherme Habreu".to_string(),
                title: "Grão de Arroz".to_string(),
                description: "Ele comprou grão por grão".to_string(),
                value: rand::thread_rng().gen_range(0.0..102.78),
                date_in: format!("{}/{}/{}", rand::thread_rng().gen_range(0..12), rand::thread_rng().gen_range(1..31), rand::thread_rng().gen_range(1900..2023)),
                date_out: format!("{}/{}/{}", rand::thread_rng().gen_range(0..12), rand::thread_rng().gen_range(1..31), rand::thread_rng().gen_range(1900..2023)),
                paid_installments: "5".to_string(),
                installments: "12".to_string(),
                status: rand::thread_rng().gen_bool(0.5)
            };
            
            vector.push(info_test.clone());
        }

        vector
    }
}