use serde::{Deserialize, Serialize};
use snowstorm::NoiseParams;

pub const NOISE_PARAMS: &'static str = "Noise_NNpsk0_25519_AESGCM_SHA256";

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageKind {
    Data,
    Heartbeat,
    Command,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub kind: MessageKind,
    pub body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Shell(ShellCommand),
    Echo(String),
    Ack,
    Shutdown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
}

pub(crate) fn create_noise_builder<'a>(key: &'a [u8]) -> snow::Builder<'a> {
    let params: NoiseParams = NOISE_PARAMS.parse().expect("invalid noise params");
    snowstorm::Builder::new(params).psk(0, &key)
}

pub(crate) fn derive_psk(password: &str) -> Vec<u8> {
    let digest = ring::digest::digest(&ring::digest::SHA256, password.as_bytes());
    digest.as_ref()[..32].to_vec()
}
