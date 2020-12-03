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

- Tests
- Error handling
- Refactoring modules

much more...

## License

MIT