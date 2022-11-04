FROM rust as builder

WORKDIR /app
COPY . /app

RUN cargo install --path=.

FROM rust
COPY --from=builder /usr/local/cargo/bin /usr/local/cargo/bin
