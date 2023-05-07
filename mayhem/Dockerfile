FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /blend
WORKDIR /blend/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /blend/fuzz/target/x86_64-unknown-linux-gnu/release/blend-fuzz /