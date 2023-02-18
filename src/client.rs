use crate::shared::{Command, Frame, MessageKind};
use bytes::Bytes;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};
use futures::{sink::SinkExt, StreamExt};
use snowstorm::{self, NoiseStream};
use std::{
    io::{self},
    time::Duration,
};
use tokio::{net::TcpStream, time::timeout};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

mod shared;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

async fn run_client(password: &str) -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:12345").await?;

    let derived_key = shared::derive_psk(password);
    let builder = shared::create_noise_builder(&derived_key);
    let keys = builder
        .generate_keypair()
        .expect("failed to generate keypair");
    let client = builder
        .local_private_key(keys.private.as_slice())
        .build_initiator()
        .expect("failed to build initiator");
    let term = Term::stdout();

    let encrypted_stream = NoiseStream::handshake(stream, client).await.unwrap();
    let mut framed = Framed::new(encrypted_stream, LengthDelimitedCodec::new());

    loop {
        let mut heartbeat = HEARTBEAT_INTERVAL;
        send_heartbeat(&mut framed).await;
        if let Ok(line) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(">")
            .allow_empty(true)
            .interact_text()
        {
            let command = line.split(" ").collect::<Vec<&str>>();
            let command = match command[0] {
                "echo" => {
                    heartbeat = Duration::from_secs(0);
                    Command::Echo(command[1].to_string())
                }
                "exec" => {
                    heartbeat = Duration::from_secs(5);
                    make_shell_command(command)
                }
                "exit" => std::process::exit(0),
                _ => {
                    heartbeat = Duration::from_secs(0);
                    Command::Ack
                }
            };
            let message = Frame {
                kind: MessageKind::Command,
                body: bincode::serialize(&command).unwrap(),
            };
            let frame = bincode::serialize(&message).unwrap();
            let data = Bytes::from(frame);
            let result = framed.send(data).await;
            match result {
                Err(e) => {
                    println!("Error: {:?}", e);
                }
                Ok(_) => {
                    framed.flush().await.expect("failed to flush");
                }
            }
        }

        while let Some(bytes) = timeout(heartbeat, framed.next()).await.ok() {
            match bytes {
                Some(b) => match b {
                    Ok(bytes) => {
                        let decoded_frame: Frame =
                            bincode::deserialize(&bytes).expect("failed to decode frame");
                        match decoded_frame.kind {
                            MessageKind::Data => {
                                let data: String = bincode::deserialize(&decoded_frame.body)
                                    .expect("failed to decode data");
                                term.write_line(&data).unwrap();
                            }
                            _ => break,
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        return Ok(());
                    }
                },
                None => break,
            }
        }
    }
}

async fn send_heartbeat(framed: &mut Framed<NoiseStream<TcpStream>, LengthDelimitedCodec>) {
    let heartbeat = Frame {
        kind: MessageKind::Heartbeat,
        body: vec![],
    };
    let heartbeat = bincode::serialize(&heartbeat).expect("failed to serialize heartbeat");
    framed.send(Bytes::from(heartbeat)).await.unwrap();
}

fn make_shell_command(command: Vec<&str>) -> Command {
    let mut cmd = shared::ShellCommand {
        command: command[1].to_string(),
        args: vec![],
    };
    for arg in command[2..].iter() {
        cmd.args.push(arg.to_string());
    }
    Command::Shell(cmd)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        std::process::exit(1);
    }

    let password = &args[1];
    run_client(password).await
}
