# MerkleMap CLI

A command-line interface (CLI) client for the [merklemap](https://www.merklemap.com/) API. This tool allows you to
search / enumerate subdomains
matching a given query and tail live subdomain discoveries from the Merklemap ingestion pipeline.

## Features

- Search for subdomains matching a specific query
- Tail live subdomain discoveries in real-time

## Installation

To install the MerkleMap CLI, you need to have Rust and Cargo installed on your system. If you don't have them
installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo installed, you can build and install the CLI by running:

```
cargo install merklemap-cli
```

This will compile the project and install the binary in your Cargo bin directory.

## Usage

The MerkleMap CLI provides two main commands:

### Search

To search for subdomains matching a query:

```
merklemap-cli search <QUERY>
```

Replace `<QUERY>` with your search term.

### Tail

To tail live subdomain discoveries:

```
merklemap-cli tail
```

## Output Format

### Search Output

The search command outputs results in the following format:

```
domain=<domain> subject_common_name=<common_name> not_before=<timestamp> human_readable_not_before=<formatted_date>
```

### Tail Output

The tail command outputs results in the following format:

```
hostname=<hostname> timestamp=<ISO8601_timestamp> human_readable_not_before=<formatted_date>
```

## Development

To build the project:

```
cargo build
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
