use std::io::{Read, Write};
use std::net::TcpStream;

mod common;

fn main() {
    let password = std::env::args()
        .nth(1)
        .expect("Password argument not found");
    let key = common::derive_key(&password);
    let mut nonce = common::derive_nonce(&password);

    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to the server");

    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim_end().as_bytes();

        if input == b"exit" {
            break;
        }

        let encrypted_input = common::encrypt_data(input, &key, &nonce);

        stream
            .write_all(&encrypted_input)
            .expect("Failed to write data to the stream");

        let mut buffer = [0u8; 4096];
        let n = stream
            .read(&mut buffer)
            .expect("Failed to read data from the stream");

        let decrypted_output =
            common::decrypt_data(&buffer[..n], &key, &nonce).expect("Failed to decrypt data");
        let output = String::from_utf8(decrypted_output).expect("Failed to decode output");
        println!("{}", output);

        common::increment_nonce(&mut nonce);
    }
}
