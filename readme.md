## Getting Started

Make sure that you have the `simple-http-server` crate installed in your machine.

```bash
    cargo install simple-http-server
```

## Development

```bash
    # This is to be done on each crate in the multi-crate repo e.g. in the forces directory
    wasm-pack build --target web
```

To see the examples locally you can run the installed **simple-http-server**

```bash
# Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/)
    simple-http-server
```
