use std::env;

use crypto::aes::KeySize::KeySize256;
use crypto::aes::{cbc_decryptor, cbc_encryptor};
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use rand::{thread_rng, Rng};

pub fn crpt(msg: String) {
    let iv = [0u8; 16];
    let key = env::var("KEYESMERALD").unwrap();
    // thread_rng().fill(&mut iv);
    // println!("iv: {:?}", iv);
    // println!("key: {:?}", key);

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

    let msg: String = unsafe{ 
        String::from_utf8_lossy(&*final_result).to_string()
    };

    println!("encrypted: {:?}", msg);

    // let mut decryptor = cbc_decryptor(KeySize256, key.as_bytes(), &iv, crypto::blockmodes::PkcsPadding);
    // let mut decrypted_result = Vec::<u8>::new();
    // let mut read_buffer = RefReadBuffer::new(&final_result);

    // loop {
    //     let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);

    //     match result {
    //         Ok(BufferResult::BufferUnderflow) => break,
    //         Ok(BufferResult::BufferOverflow) => {
    //             decrypted_result.extend(write_buffer.take_read_buffer().take_remaining());
    //         }
    //         Err(_) => {},
    //     }
    // }

    // decrypted_result.extend(write_buffer.take_read_buffer().take_remaining());

    // println!("decrypted: {:?}", String::from_utf8_lossy(&decrypted_result));
}

pub fn get_key() -> String {
    let characters: String =
        "AaBbCcDdEeFfGgHhIiJjKkLlMnOoPpQqRrSsTtUuVvWwXxYyZz1234567890!@#$%*():><.,}{^`?~|^"
            .to_string();

    let mut key = String::new();

    for _ in 0..10 {
        let index = thread_rng().gen_range(0..characters.len());
        key.push(characters.chars().nth(index).unwrap());
    }

    key
}
