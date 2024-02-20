use chrono::NaiveDate;

use crate::prelude::chrono;

pub fn is_date(text: &str) -> bool{
    let response = NaiveDate::parse_from_str(text, "%Y").is_ok();

    // if !response{
    //     let response = NaiveDate::parse_from_str(text, "%M").is_ok();
    // }else{

    // }
    println!("{}", response);

    response
}