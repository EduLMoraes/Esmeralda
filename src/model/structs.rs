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

#[derive(Clone)]
pub struct Info{
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
impl Info{
    pub fn test() -> Vec<Info>{
        let info_test = Info{
            debtor: "Guilherme Habreu".to_string(),
            title: "Grão de Arroz".to_string(),
            description: "Ele comprou grão por grão".to_string(),
            value: 0.50,
            date_in: "12/03/2009".to_string(),
            date_out: "12/03/2010".to_string(),
            paid_installments: "5".to_string(),
            installments: "12".to_string(),
            status: true
        };

        let mut vector: Vec<Info> = Vec::new();

        for _ in 0..200{
            vector.push(info_test.clone());
        }

        vector
    }
}