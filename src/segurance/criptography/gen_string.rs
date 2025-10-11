use rand::{thread_rng, Rng};

pub fn gen_string(size: i32, range: &[u8; 2]) -> String {
    let mut string = String::new();

    for _ in 0..size {
        let index: u8 = thread_rng().gen_range(range[0]..range[1]);
        string.push(index as char);
    }

    string
}
