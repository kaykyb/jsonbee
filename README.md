# jsonbee

`jsonbee` is a command-line utility written in Rust to encode JSON into Bencode and decode Bencode back into JSON.

## Features

- **Encode JSON to Bencode**: Convert JSON data into Bencode format.
- **Decode Bencode to JSON**: Convert Bencode data back into JSON format.

## Installation

To install `jsonbee`, you need to have Rust installed on your machine. If Rust is not already installed, you can install it via [rustup](https://rustup.rs/), which sets up Rust and `cargo` on your system.

Once Rust is installed, you can build and install `jsonbee` by cloning the repository and using `cargo`:

```bash
git clone https://github.com/kaykyb/jsonbee.git
cd jsonbee
cargo build --release
```

The executable will be generated in the target/release directory. Optionally, you can install it directly to your Cargo bin directory:

```bash
cargo install --path .
```

## Usage

jsonbee can be used directly from the command line. Here are the basic commands for encoding and decoding:

### Encoding JSON to Bencode

```bash
echo '{"name": "John", "age": 30}' | ./target/release/jsonbee encode
```

### Decoding Bencode to JSON

```bash
echo 'd4:name4:John3:agei30ee' | ./target/release/jsonbee decode
```
