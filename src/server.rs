use anyhow::Result;
use bytes::Bytes;
use snowstorm::NoiseStream;
use snowstorm::{self};
use std::{error::Error, process::Stdio};
use tokio::net::{TcpListener, TcpStream};
mod shared;
use futures::{sink::SinkExt, StreamExt};
use std::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

async fn do_echo(s: String, framed: &mut Framed<NoiseStream<TcpStream>, LengthDelimitedCodec>) {
    framed
        .send(Bytes::from(
            bincode::serialize(
                &(shared::Frame {
                    kind: shared::MessageKind::Data,
                    body: bincode::serialize(&(format!("echo: {}", s))).unwrap(),
                }),
            )
            .unwrap(),
        ))
        .await
        .expect("failed to send response");
}

fn do_ack() {
    println!("Received ack");
}

async fn do_shell(
    cmd: shared::ShellCommand,
    framed: &mut Framed<NoiseStream<TcpStream>, LengthDelimitedCodec>,
) {
    println!("Received shell command: {:?}", cmd);
    let output = Command::new(cmd.command)
        .args(cmd.args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute process");

    framed
        .send(Bytes::from(
            bincode::serialize(
                &(shared::Frame {
                    kind: shared::MessageKind::Data,
                    body: bincode::serialize(&output.stdout).unwrap(),
                }),
            )
            .unwrap(),
        ))
        .await
        .expect("failed to send response");
}

async fn do_shutdown(framed: &mut Framed<NoiseStream<TcpStream>, LengthDelimitedCodec>) {
    println!("Received shutdown command");
    framed
        .send(Bytes::from(
            bincode::serialize(
                &(shared::Frame {
                    kind: shared::MessageKind::Data,
                    body: bincode::serialize(&"Server shutting down...").unwrap(),
                }),
            )
            .unwrap(),
        ))
        .await
        .expect("failed to send response");
    std::process::exit(0);
}

async fn handle_connection(stream: TcpStream, responder: snow::HandshakeState) {
    match NoiseStream::handshake(stream, responder).await {
        Ok(stream) => {
            println!("handshake complete");
            let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
            while let Some(bytes) = framed.next().await {
                match bytes {
                    Ok(bytes) => {
                        let decoded_frame: shared::Frame =
                            bincode::deserialize(&bytes).expect("failed to decode frame");
                        match decoded_frame.kind {
                            shared::MessageKind::Command => {
                                let command: shared::Command =
                                    bincode::deserialize(&decoded_frame.body)
                                        .expect("failed to decode command");
                                match command {
                                    shared::Command::Echo(s) => do_echo(s, &mut framed).await,
                                    shared::Command::Ack => do_ack(),
                                    shared::Command::Shell(cmd) => do_shell(cmd, &mut framed).await,
                                    shared::Command::Shutdown => do_shutdown(&mut framed).await,
                                }
                            }
                            shared::MessageKind::Heartbeat => {
                                println!("Received heartbeat");
                            }
                            // Ignore data messages
                            _ => return,
                        }
                    }
                    Err(e) => {
                        match e.kind() {
                            tokio::io::ErrorKind::UnexpectedEof => {
                                println!("Connection closed");
                            }
                            std::io::ErrorKind::ConnectionReset => {
                                println!("Connection reset");
                            }
                            _ => {
                                println!("Error: {}", e);
                            }
                        }
                        break;
                    }
                }
            }
        }
        Err(_) => return,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        std::process::exit(1);
    }

    let password = args[1].clone();
    run_server(password).await?;

    Ok(())
}

async fn run_server(password: String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
        let derived_key = shared::derive_psk(password.as_str());
        let builder = shared::create_noise_builder(&derived_key);
        let keys = builder
            .generate_keypair()
            .expect("failed to generate keypair");
        let responder = builder
            .local_private_key(keys.private.as_slice())
            .build_responder()?;

        tokio::spawn(async move {
            handle_connection(stream, responder).await;
        });
    }
}
