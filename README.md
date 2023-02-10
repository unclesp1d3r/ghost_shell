# Ghost_Shell

![GitHub](https://img.shields.io/github/license/unclesp1d3r/ghost_shell)
![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/ghost_shell)
![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/ghost_shell?style=social)
![GitHub last commit](https://img.shields.io/github/last-commit/unclesp1d3r/ghost_shell)
![Maintenance](https://img.shields.io/maintenance/yes/2022)

This project implements a secure shell client and server using the ChaCha20Poly1305 encryption algorithm. The encryption key is derived from a password passed on the command line.

## Requirements

Rust (1.48 or later)

## Building and Running

To build the project, run the following command:

```bash
cargo build --release
```

To run the server, use the following command and pass in the password as the first argument:

```bash
./target/release/server <password>
```

To run the client, use the following command and pass in the password as the first argument:

```bash
./target/release/client <password>
```

Once connected to the server, the client can send shell commands to the server, and the server will execute the command and return the output to the client. The client can also exit the connection by sending the exit command.

## Security

To ensure secure communication between the client and server, all data sent between them is encrypted using the ChaCha20Poly1305 encryption algorithm. The encryption key is derived from a password shared between the client and server, and a unique nonce is generated for each communication session to prevent replay attacks.

## License

This project is licensed under the MIT license. See the LICENSE file for details.
