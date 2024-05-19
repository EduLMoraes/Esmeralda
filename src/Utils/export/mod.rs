use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use printpdf::*;

use crate::prelude::model::List::ListCount;

pub mod csv;
pub mod html;
pub mod mkdir;
pub mod pdf;
pub mod svg;
