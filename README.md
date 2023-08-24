# Axum Handler dependency injection example

This repo is the companion code of 
https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/

# Usage

`$ cargo run` will start four apps. Each one using a different mechanism to
provide State to the handlers.

- http://localhost:9090: struct static
- http://localhost:9091: generic static
- http://localhost:9092: trait static
- http://localhost:9093: dynamic with trait object
