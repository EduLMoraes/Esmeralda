use super::*;

pub fn encrpt(msg: String) -> String {
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

    let msg: String = String::from_utf8_lossy(&final_result).to_string();

    msg
}
