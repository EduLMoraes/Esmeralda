#[cfg(test)]
mod test_people {
    use chrono::Utc;

    use crate::model::errors::{ErrorLog, PeopleError};
    use crate::model::People::People;

    #[test]
    pub fn test_cpf_valid() {
        use rand::Rng;

        for _ in 0..300 {
            let mut rng = rand::thread_rng();
            let mut cpf: Vec<u32> = (0..9).map(|_| rng.gen_range(0..=9)).collect();

            let mut soma: u32 = cpf
                .iter()
                .enumerate()
                .map(|(i, &v)| v * (10 - i as u32))
                .sum();
            let primeiro_dv = if soma % 11 < 2 { 0 } else { 11 - (soma % 11) };
            cpf.push(primeiro_dv);

            soma = cpf
                .iter()
                .enumerate()
                .map(|(i, &v)| v * (11 - i as u32))
                .sum();
            let segundo_dv = if soma % 11 < 2 { 0 } else { 11 - (soma % 11) };
            cpf.push(segundo_dv);

            let cpf_test = format!(
                "{}{}{}.{}{}{}.{}{}{}-{}{}",
                cpf[0],
                cpf[1],
                cpf[2],
                cpf[3],
                cpf[4],
                cpf[5],
                cpf[6],
                cpf[7],
                cpf[8],
                cpf[9],
                cpf[10]
            );

            assert!(People::valid_cpf(&cpf_test));
        }
    }

    #[test]
    pub fn test_cpf_invalid() {
        assert!(!People::valid_cpf("000000000000000"));
        assert!(!People::valid_cpf("0000.000.000-00"));
        assert!(!People::valid_cpf("000.000.000-00"));
        assert!(!People::valid_cpf("000@000.000-00"));
        assert!(!People::valid_cpf("000@000@000-00"));
        assert!(!People::valid_cpf("000@000-000-00"));
        assert!(!People::valid_cpf("0001000-000-00"));
        assert!(!People::valid_cpf("000e000-000-00"));
        assert!(!People::valid_cpf("000e000e000e00"));
        assert!(!People::valid_cpf("000.000e000e00"));
        assert!(!People::valid_cpf("000.000.000e00"));
        assert!(!People::valid_cpf("001.000.000100"));
        assert!(!People::valid_cpf("001.000.000100"));
        assert!(!People::valid_cpf("02e.000.000-00"));
    }

    #[test]
    pub fn test_create_people_err() {
        let test_people = People::from(
            String::from("abcd-4-jklm-pqrls-mw3f"),
            String::from("Jhon"),
            1.400,
            String::from("(99) 99999-9999"),
            Utc::now().date_naive(),
            String::from("555555555555"),
            String::from("000.000.000-01"),
            String::from("Molton"),
        );

        assert!(test_people.is_err());
        assert_eq!(
            test_people.err(),
            Some(PeopleError::CPFInvalid(ErrorLog {
                title: "This cpf is invalid!",
                code: 902,
                file: "People.rs",
            }))
        );
    }
}
