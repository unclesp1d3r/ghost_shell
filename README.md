# Ghost_Shell

![GitHub](https://img.shields.io/github/license/unclesp1d3r/ghost_shell)
![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/ghost_shell)
![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/ghost_shell?style=social)
![GitHub last commit](https://img.shields.io/github/last-commit/unclesp1d3r/ghost_shell)
![Maintenance](https://img.shields.io/maintenance/yes/2023)

Ghost Shell is a lightweight and fast remote shell that provides secure communication between a client and a server. The communication channel is encrypted using the Noise_NNpsk0_25519_AESGCM_SHA256 noise protocol and derives the pre-shared key using SHA256.

The noise protocol provides a framework for building encrypted channels, and Ghost Shell uses it to secure communication between the client and server. The pre-shared key is derived using the SHA256 hash function, ensuring secure key exchange. The encryption algorithm, AES-GCM, provides authenticated encryption with additional data (AEAD), which ensures data integrity, authenticity, and confidentiality. These measures provide end-to-end security, protecting against eavesdropping, tampering, and forgery of messages.

## Requirements

Rust (1.48 or later)

## Building and Running

To build the project, run the following command:

```bash
git clone https://github.com/unclesp1d3r/ghost_shell.git
cd ghost_shell
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

### Supported Commands

Ghost Shell currently supports the following commands:

- `echo`: Echoes back the input provided as an argument.
- `exec`: Executes a command on the server and returns the output.
- `exit`: Exits the remote shell and terminates the connection.

Additional commands may be added in the future.

## Security

Ghost Shell uses the Noise_NNpsk0_25519_AESGCM_SHA256 noise protocol to encrypt the communication channel between the client and the server. This protocol provides end-to-end security, protecting against eavesdropping, tampering, and forgery of messages.

The Noise_NNpsk0_25519_AESGCM_SHA256 protocol utilizes Elliptic-curve Diffie-Hellman key agreement with Curve25519, a strong and efficient key exchange mechanism. The pre-shared key is derived using the SHA256 hash function, ensuring secure key exchange. The encryption algorithm, AES-GCM, provides authenticated encryption with additional data (AEAD), which ensures data integrity, authenticity, and confidentiality.

## Caveats

While we have taken steps to ensure the security of the communication channel, it is important to note that no security solution is foolproof. Therefore, Ghost Shell may be vulnerable to attacks such as brute-force password cracking or implementation-specific attacks.

Furthermore, Ghost Shell is a work in progress and has not been audited in any way. Therefore, we do not recommend using it in a production environment without first conducting a thorough security review.

## Contributions

We welcome contributions to Ghost Shell in the form of pull requests. If you find a bug, or have an idea for a new feature, we encourage you to submit a pull request. Before submitting a pull request, please ensure that your code adheres to the Rust programming language style guide and that it passes all tests.

To submit a pull request, fork the repository, make your changes, and then submit a pull request. We will review your changes and provide feedback. If your changes are accepted, we will merge them into the main branch.

## Disclaimer

Please note that Ghost Shell is a hobby project created for educational and research purposes. It is not intended to be used for malicious purposes, and we do not condone any illegal or unethical activities using this software.

## License

This project is licensed under the MIT license. See the LICENSE file for details.
