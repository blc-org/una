# Node bindings for UNA

Requires Node.js >=10.

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
