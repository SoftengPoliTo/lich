# Light Firwmare

A simple light firmware implemented with the
[ascot](https://github.com/SoftengPoliTo/ascot) framework.

## Building

```console
cargo build [--release]
```

To build a firmware for `Linux` systems with the `musl` library:

```console
cargo build [--release] --target x86_64-unknown-linux-musl
```

The `--release` option configures a build with the Rust `release` profile,
which enables all time and memory optimizations for the considered architecture.

## Cross-compilation for `aarch64` architecture (ARM64)

Install a binary named [cross](https://github.com/cross-rs/cross) which allow
to easily cross-compile Rust projects using Docker, without messing with
custom `Dockerfile`s.

```console
cargo install -f cross --git https://github.com/cross-rs/cross
```

The `git` option allows to use the most recent `docker` images. Even if the
`cross` project is currently maintained, it does not release constantly,
so the published versions become obsolete fast enough.

To build a firmware for `Linux` systems running on `ARM64` architecture:

```console
cross build [--release] --target=aarch64-unknown-linux-gnu
```

To build a firmware with the `musl` library for `Linux` systems running on
`ARM64` architecture:

```console
cross build [--release] --target=aarch64-unknown-linux-musl
```

The `--release` option configures a build with the Rust `release` profile,
which enables all time and memory optimizations for `ARM64` architecture.

## Running the server

The server runs on `localhost` and listens to port `3000`. To run the server,
it is necessary to pass as input the server's `hostname` and the discovery 
process `domain`.

```console
cargo run -- --hostname HOSTNAME --domain DOMAIN
```

where `HOSTNAME` corresponds to the chosen server's hostname, while `DOMAIN`
corresponds to the domain sought by a client during the network discovery
process.

At server startup, an initial message signalling its effective execution
is printed.

```
Starting server...
```
