# challonge-rs 
[![Crates badge](https://img.shields.io/crates/v/challonge.svg)](https://crates.io/crates/challonge)
[![CI](https://github.com/vityafx/challonge-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/vityafx/challonge-rs/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/challonge/badge.svg)](https://docs.rs/challonge)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)


Client library for the [Challonge](https://challonge.com) REST API.

## Usage
 1. Log in to Challonge with `Challonge::new`.
 2. Call API methods to interact with the service.

## Documentation
[Challonge API documentation](http://api.challonge.com/ru/v1/documents).

## Features
- `default` - uses `rustls` backend for `reqwest`.
- `default-tls` - uses `default-tls` backend for `reqwest`.

## Examples
See the `examples` directory in the source tree.

