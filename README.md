# Ghost_Shell

![GitHub](https://img.shields.io/github/license/unclesp1d3r/ghost_shell)
![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/ghost_shell)
![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/ghost_shell?style=social)
![GitHub last commit](https://img.shields.io/github/last-commit/unclesp1d3r/ghost_shell)
![Maintenance](https://img.shields.io/maintenance/yes/2022)

This project implements a simple reverse shell that communicates over a TCP connection and encrypts all data using AES encryption. The encryption key is derived from a password passed on the command line.

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

## Implementation Details

The encryption key is derived using SHA-256, and the nonce is derived using a truncated SHA-256 hash of the password. The encryption and decryption of data is done using the AES-GCM algorithm. The client and server communicate by sending encrypted commands and outputs back and forth. The nonce is incremented after each message to ensure that the same nonce is not used twice.

## Security

This project is for educational purposes only and is not suitable for use in a production environment. The security of the encryption and key derivation is dependent on the strength of the password, so it is important to choose a strong and unique password. Additionally, the code has not been thoroughly reviewed for security vulnerabilities, so there may be unknown issues present.

## License

This project is licensed under the MIT license. See the LICENSE file for details.
