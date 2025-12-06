use std::env;

use crypto::aes::cbc_encryptor;
use crypto::aes::KeySize::KeySize256;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

mod encrypt;
mod gen_string;
pub use encrypt::*;
pub use gen_string::*;
