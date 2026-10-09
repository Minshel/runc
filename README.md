<div align="center">

<h1>RunC</h1>
<h6>It's a simple tool for executing a large number of commands using a single command.</h6>

[![Rust](https://img.shields.io/badge/Rust-1.99.0-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)

<div align="left">

## ![runinfo.command](https://github.com/Minshel/runc/blob/main/runinfo.command) example

```toml
[main]
cargo clean;
cargo build;

[dependencies-shell]
nix-shell -p rustc cargo ...;
```

## Command Example

runc [GROUP] [CONFIG_FILE]

CONFIG_FILE argument is optional
```sh
runc dependencies-shell
```
