use clap::Parser;
use rand::thread_rng;
use std::io::{self, Read, Write};
use std::net::TcpStream;

mod common;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "You")]
struct Opts {
    #[clap(
        short,
        long,
        value_name = "PASSWORD",
        help = "Password for the connection"
    )]
    password: String,

    #[clap(
        short,
        long,
        value_name = "ADDRESS",
        help = "Address of the server to connect to"
    )]
    address: String,
}

fn handle_server(mut stream: std::net::TcpStream, password: &str) {
    let key = common::derive_key(password);
    let nonce = derive_nonce();
    stream.write_all(&nonce).unwrap();
    stream.flush().unwrap();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim_end() == "exit" {
            break;
        }

        let buffer = buffer.trim_end().as_bytes().to_vec();
        let buffer = common::encrypt_data(&buffer, &key, &nonce).unwrap();
        stream.write_all(&buffer).unwrap();
        stream.flush().unwrap();

        let mut response = [0u8; 65536];
        let n = stream.read(&mut response).unwrap();
        let decrypted = common::decrypt_data(&response[..n], &key, &nonce).unwrap();
        println!("{}", String::from_utf8(decrypted).unwrap());
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let stream = TcpStream::connect(opts.address).unwrap();
    handle_server(stream, &opts.password);
}

fn derive_nonce() -> chacha20poly1305::Nonce {
    let mut nonce_bytes = [0u8; common::NONCE_LEN];
    rand::RngCore::fill_bytes(&mut thread_rng(), &mut nonce_bytes);
    *chacha20poly1305::Nonce::from_slice(&nonce_bytes)
}
