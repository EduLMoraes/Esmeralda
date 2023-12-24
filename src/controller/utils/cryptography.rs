use std::env;

use crypto::aes::KeySize::KeySize256;
use crypto::aes::cbc_encryptor;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use rand::{thread_rng, Rng};

#[allow(dead_code)]
/// Encrypts a message using the AES algorithm in CBC mode with a 256-bit key.
///
/// # Arguments
///
/// * `msg` - The message to be encrypted.
///
/// # Example
///
/// ```
/// use std::env;
/// use crypto::symmetriccipher::SymmetricCipherError;
/// use crypto::buffer::{RefReadBuffer, RefWriteBuffer, BufferResult};
/// use crypto::aes::{cbc_encryptor, KeySize::KeySize256};
///
/// #[allow(dead_code)]
/// pub fn encrpt(msg: String) -> String{
///     let iv = [0u8; 16];
///     let key = env::var("KEYESMERALD").unwrap();
///
///     let mut encryptor = cbc_encryptor(
///         KeySize256,
///         key.as_bytes(),
///         &iv,
///         crypto::blockmodes::PkcsPadding,
///     );
///
///     let mut final_result = Vec::<u8>::new();
///     let mut read_buffer = RefReadBuffer::new(msg.as_bytes());
///     let mut buffer = [0; 4096];
///     let mut write_buffer = RefWriteBuffer::new(&mut buffer);
///
///     loop {
///         let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);
///
///         match result {
///             Ok(BufferResult::BufferUnderflow) => break,
///             Ok(BufferResult::BufferOverflow) => {
///                 final_result.extend(write_buffer.take_read_buffer().take_remaining());
///             }
///             Err(_) => {}
///         }
///     }
///
///     final_result.extend(write_buffer.take_read_buffer().take_remaining());
///
///     let msg: String = String::from_utf8_lossy(&*final_result).to_string();
///
///     msg
/// }
/// ```
pub fn encrpt(msg: String) -> String{
    let iv = [0u8; 16];
    let key = env::var("KEYESMERALD").unwrap();

    let mut encryptor = cbc_encryptor(
        KeySize256,
        key.as_bytes(),
        &iv,
        crypto::blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(msg.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);

        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => {
                final_result.extend(write_buffer.take_read_buffer().take_remaining());
            }
            Err(_) => {}
        }
    }

    final_result.extend(write_buffer.take_read_buffer().take_remaining());

    let msg: String = String::from_utf8_lossy(&*final_result).to_string();

    msg
}

/// Generates a random key of length 1024 characters.
///
/// # Example
///
/// ```
/// let key = get_key();
/// println!("Generated key: {}", key);
/// ```
///
/// # Returns
///
/// A randomly generated key of length 1024 characters.
pub fn get_key() -> String {
    let mut key = String::new();

    for _ in 0..1024 {
        let index: u8 = thread_rng().gen_range(33..126);
        key.push(index as char);
    }
    println!("{}", key);
    key
}
