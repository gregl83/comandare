# comandare

TCP Command Execution

## Problem Statement

Network ports enable computers to interact around the world but unintentional open ports wreak havoc on system security.

## Intent

Demonstrate how a program can open a port enabling remote access; otherwise, known as a backdoor.

## Goal

Reinforce security intuition with network port management.

## Installation

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Git clone this repository.
3. From cloned repository, run `cargo install --path=.`
4. That's it! `comandare` should now be installed on your system.

## Usage

Run `comandare` to see command execution format.

Supported modes:

- Client (uses command argument)
- Server (doesn't use command argument)

## Improvements

- Auth connections
- Documentation (code and usage)
- Tests
- Error handling
- Refactoring modules
- Command parsing module (handling pipe, etc)

much more...

## Caution

This package is not intended on being deployed or used in an environment other than for educational testing; thus, sacrifices have been made to simply assemble a proof of concept. NOT to be used maliciously. 

The TCP Server does not implement any form of Auth leaving your network security as the sole guardian of your data.

Opening TCP Ports on a system *can* be bad, use at your own risk!

## License

[MIT](LICENSE)
