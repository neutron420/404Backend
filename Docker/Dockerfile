FROM rust:1.73 as builder

WORKDIR /usr/src/app

COPY ../ .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/backend-rust /usr/local/bin/backend-rust

CMD ["backend-rust"]