FROM rust:alpine3.21 AS builder

RUN apk add --no-cache musl-dev

COPY src/ /build/src/
COPY Cargo.toml Cargo.lock /build/

WORKDIR /build
RUN cargo build --release

FROM alpine:3.22
COPY --from=builder /build/target/release/pass-rs /usr/local/bin
ENTRYPOINT [ "pass-rs" ]
CMD [ "--help" ]
