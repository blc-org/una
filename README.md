# UNA - Universal Node API

Una is a Lightning network node wrapper for multiple backends and for multiple programming languages.

🚧 Una is still in development.

## Supported actions
 - [x] Get node info
 - [x] Create invoice
 - [x] Pay invoice
 - [ ] Get invoice
 - [ ] Decode invoice
 - [ ] Invoice events

## Supported backends
 - [x] LND (REST)
 - [x] Core Lightning
 - [x] Eclair (REST) (>= v0.6.2)
 - [ ] LndHub
 - [ ] LndHub.go V2
 - [ ] LNBits

## Supported programming languages
 - [x] Rust
 - [x] Node.js
 - [x] Python

Want another action, backend or programming language? [Open an issue](https://github.com/blc-org/una/issues/new)

> ⚠️ The following documentation is concerning the Rust version which is the code base for all other bindings.
> 
> Other programming languages documentations can be found in the `bindings` folder.
> - [Node.js](./bindings/una-js/README.md)
> - [Python](./bindings/una-python/README.md)

## Usage (Rust)

🚧 TODO

## Usage (CLI)

### Build
```sh
cargo run --package una-proto-builder
cargo build --bin una-cli
```

### Choose your backend
#### LND

```sh
una-cli -- --backend LndRest --url https://127.0.0.1:8081 --macaroon HEX_MACAROON --tls_certificate HEX_CERTIFICATE
```

#### Core Lightning

```sh
una-cli -- --backend ClnGrpc --url https://127.0.0.1:11002 --tls_certificate HEX_TSL_CERTIFICATE --tls_client_key HEX_CLIENT_KEY --tls_client_certificate HEX_TLS_CLIENT_CERTIFICATE
```

#### Eclair

```sh
una-cli -- --backend EclairRest --url http://127.0.0.1:8283 --username USERNAME  --password PASSWORD
```

### Actions
#### Get node info
```sh
una-cli ... info
```

#### Create invoice
```sh
una-cli ... createinvoice 1000 description
```

#### Get invoice
```sh
una-cli ... getinvoice 4d961f2bdda9cb9c4c64739e928ca06d2921357fe437a59214809828bba0dde2
```