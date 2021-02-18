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
2. Git clone this repository
3. From cloned repository, run:

   `cargo install --path=.`

4. That's it, `comandare` should now be installed on your system!

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

## Model

Comandare uses a server target model which is largely mitigated by any standard firewall. Incoming traffic in most configurations is rightfully blocked. Thus, binding a TCP Server to a specific port might work in a local network but be blocked from remote access over the internet or networks backed by one or more firewalls. A more effective approach, since TCP is a two-way communication protocol, is to initiate a client target model with event-driven connection attempts.

## Caution

This package is not intended on being deployed or used in an environment other than for educational testing; thus, sacrifices have been made to simply assemble a proof of concept. NOT to be used maliciously. 

The TCP Server does not implement any form of Auth leaving your network security as the sole guardian of your data.

Opening TCP Ports on a system *can* be bad, use at your own risk!

## Detection

An academic background in computer networking is worth its weight in gold while evaluating potential network security threats. While this is true, there are a few tools that can be used to identify potential threats.

- [Wireshark](https://www.wireshark.org/) network analyzer provides comprehensive details on network activity.
- [netstat](https://en.wikipedia.org/wiki/Netstat) networking utility provides insight into port usage with minimal configuration.

Useful `netstat` command to get the ball rolling:

```bash
netstat -tulnp
```

There are many other useful tools available, but between `Wireshark` and `netstat` one can easily evaluate network activity and identify suspicious actors.

## Final Words

Hopefully this package can increase intuition on backdoor implementations indirectly helping secure systems.

## License

[MIT](LICENSE)
