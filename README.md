# clean-code
A very bad user verfication

First you need to install rust.
For this use : https://www.rust-lang.org/fr

# Run this api

## Debug
```bash
    cargo run
```

## Test
```bash
    cargo test
```

## Build usable executable
```bash
    cargo build --release
```

## Run an usable executable
```bash
    cargo run --release
```

# Test This api

For this open in your dev environement :
http://127.0.0.1:8000/doc/index.html

We use Swagger with rocket_okapi for generate openapi file

# Generate documentation

```bash
    cargo doc
```