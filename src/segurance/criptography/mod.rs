use crypto::{
    aes::{cbc_encryptor, KeySize::KeySize256},
    buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer},
};
use std::env;

mod encrypt;
mod gen_string;
pub use encrypt::*;
pub use gen_string::*;
