use clap::Parser;
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
    let nonce = common::derive_nonce();
    stream.write_all(&nonce).unwrap();
    stream.flush().unwrap();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let command = match line.trim() {
            "ls" => common::Command::Ls,
            "pwd" => common::Command::Pwd,
            "quit" => {
                break;
            }
            _ => {
                let mut parts = line.splitn(2, ' ');
                let command = parts.next().unwrap();
                let arg = parts.next().unwrap_or("");
                match command {
                    "cd" => common::Command::Cd {
                        path: arg.to_owned(),
                    },
                    "echo" => common::Command::Echo {
                        message: arg.to_owned(),
                    },
                    "exec" => common::Command::Exec {
                        command: arg.to_owned(),
                    },
                    _ => {
                        eprintln!("Invalid command: {}", command);
                        continue;
                    }
                }
            }
        };

        let plaintext = bincode::serialize(&command).expect("error encoding command");

        let buffer = common::encrypt_data(&plaintext, &key, &nonce).expect("Failed to encrypt");
        stream.write_all(&buffer).unwrap();
        stream.flush().unwrap();

        let mut response = [0u8; 65536];
        let n = stream.read(&mut response).unwrap();
        let decrypted =
            common::decrypt_data(&response[..n], &key, &nonce).expect("Failed to decrypt");
        println!("{}", String::from_utf8(decrypted).unwrap());
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let stream = TcpStream::connect(opts.address).unwrap();
    handle_server(stream, &opts.password);
}
