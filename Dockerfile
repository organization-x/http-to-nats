# syntax=docker/dockerfile:1

FROM rust:alpine
WORKDIR /app

COPY --link . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

EXPOSE 3000

CMD ["/app/target/release/streaming-server"]