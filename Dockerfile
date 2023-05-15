FROM rust:alpine3.12 as rust_alpine

COPY src/ /build/src/
COPY Cargo.toml Cargo.lock /build/

WORKDIR /build
RUN cargo build --release

FROM alpine:3.18 as tool
COPY --from=rust_alpine /build/target/release/pass-rs /usr/local/bin
ENTRYPOINT [ "pass-rs" ]
CMD [ "--help" ]
