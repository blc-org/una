# Python bindings for UNA

Requires Python >=3.7.

## Build

### Setup environment

```shell
$ cd bindings/una-python
$ python -m venv .venv
$ source .venv/bin/activate
$ pip install -U pip maturin
```

### Build the package

```shell
$ maturin develop
```

### Test

You can try the build by running the test script (temporary, until automated tests). Replace the `config` values by your own node credentials (LND needs an admin macaroon).

```shell
$ python test.py
```
