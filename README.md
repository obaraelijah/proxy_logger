# proxy_logger

[![Crates.io][crates-badge]][crates-url]
![Rust version][rust-version]
![License][license-badge]
[![Workflow Status][workflow-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/proxy_logger.svg
[crates-url]: https://crates.io/crates/proxy_logger
[license-badge]: https://img.shields.io/crates/l/proxy_logger.svg
[workflow-badge]: https://github.com/qwerty541/logged-tcp-proxy/workflows/check/badge.svg
[actions-url]: https://github.com/qwerty541/logged-tcp-proxy/actions
[rust-version]: https://img.shields.io/badge/rust-1.74.1%2B-lightgrey.svg?logo=rust

## Description

This repository provides a command line interface for proxying TCP connections with payload output into the console. Payload output can be formatted in different ways: hexadecimal (lowercase and uppercase), decimal, octal and binary.

## Installation

### From crates.io (Recommended)

Run the following command and wait until the crate is compiled:

```sh
$ cargo install proxy_logger
```

Now you can run compiled binary:

```sh
$ proxy_logger --bind-listener-addr 127.0.0.1:20502 --remote-addr 127.0.0.1:20582
```

### From git repository

Run the following command and wait until the crate is compiled:

```sh
$ cargo install --git https://github.com/obaraelijah/proxy_logger.git --tag v0.1.1 proxy_logger
```

Also you can remove tag option to install the latest development version.

Now you can run compiled binary:

```sh
$ proxy_logger --bind-listener-addr 127.0.0.1:20502 --remote-addr 127.0.0.1:20582
```
## Options

Below is a list of currently supported options.

```
$ proxy_logger --help
Command line interface for proxying TCP connections with payload output into console which can be formatted different ways.

Usage: proxy_logger [OPTIONS] --bind-listener-addr <BIND_LISTENER_ADDR> --remote-addr <REMOTE_ADDR>

Options:
  -l, --level <LEVEL>
          Application logging level [default: debug] [possible values: trace, debug, info, warn, error, off]
  -b, --bind-listener-addr <BIND_LISTENER_ADDR>
          Address on which TCP listener should be binded
  -r, --remote-addr <REMOTE_ADDR>
          Address of remote server
  -t, --timeout <TIMEOUT>
          Incoming connection reading timeout [default: 60]
  -f, --formatting <FORMATTING>
          Formatting of console payload output, [default: lowerhex] [possible values: decimal, lowerhex, upperhex, binary, octal]
  -s, --separator <SEPARATOR>
          Console payload output bytes separator [default: :]
  -p, --precision <PRECISION>
          Timestamp precision [default: seconds] [possible values: seconds, milliseconds, microseconds, nanoseconds]
  -h, --help
          Print help
  -V, --version
          Print version
```

