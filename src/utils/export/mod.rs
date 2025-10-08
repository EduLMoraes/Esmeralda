use crate::prelude::model::List::ListCount;
use std::{
    fs,
    fs::{create_dir_all, File},
    io::Write,
};

pub mod csv;
pub mod html;
pub mod mkdir;
pub mod svg;
