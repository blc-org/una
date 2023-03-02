# UNA - Universal Node API

Una is a Lightning network node wrapper for multiple backends and for multiple programming languages, written in Rust.

🚧 Una is still in development.

Requires Node.js >=10.

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

## Usage 🚧

### Installation
```sh
npm install una-wrapper
```
### Import
```js
import { Node } from "una-wrapper";
```

### Choose your backend

#### LND

```js
const config = {
    url: "https://127.0.0.1:8081",
      macaroon:
        "0201036c6e6...5dcefddb97199d",
      tls_certificate:
        "2d2d2d2d2d4...2d2d2d2d2d0a",
};
const node = new Node("LndRest", config);
```

#### Core Lightning

```js
const config = {
    url: "https://localhost:11002",
      tls_certificate:
        "2d2d2d2d2d42...d2d2d0d0a",
      tls_client_certificate:
        "2d2d2d2d2d42...d2d2d0d0a",
      tls_client_key:
        "2d2d2d2d2d42...d2d2d0d0a",
};
const node = new Node("ClnGrpc", config);
```

#### Eclair

```js
const config = {
    url: "http://127.0.0.1:8283",
    username: "",
    password: "eclairpw",
};
const node = new Node("EclairRest", config);
```

### Actions
#### Get node info
```js
node.getInfo()
    .then((info) => console.log(info))
    .catch((err) => console.log(err.message, err.code));
// or
const info = await node.getInfo();
```

#### Create invoice
```js
const invoice = {
  amount: 10,
  description: "test napi",
};

node.createInvoice(invoice)
    .then((invoice) => console.log(invoice))
    .catch((err) => console.log(err.message, err.code));
// or
const invoice = await node.createInvoice(invoice);
```

#### Pay invoice
```js
const invoice = { payment_request: "lnbcrt10...cagpa8myt0" };

node.payInvoice(invoice)
    .then((result) => console.log(result))
    .catch((err) => console.log(err.message, err.code));
// or
const invoice = await node.payInvoice(invoice);
```

#### Get invoice
```js
🚧
```

## Build

### Setup environment

```shell
$ cd bindings/una-js
$ yarn install
```

### Build the package

```shell
$ yarn build
```

### Test

You can try the build by running the test script (temporary, until automated tests). Replace the `config` values by your own node credentials (LND needs an admin macaroon).

```shell
$ node test.mjs
```
