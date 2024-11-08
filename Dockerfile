FROM rust:alpine3.19 as builder

COPY src/ /build/src/
COPY Cargo.toml Cargo.lock /build/

WORKDIR /build
RUN cargo build --release

FROM alpine:3.20
COPY --from=builder /build/target/release/pass-rs /usr/local/bin
ENTRYPOINT [ "pass-rs" ]
CMD [ "--help" ]
