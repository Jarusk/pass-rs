FROM rust:alpine3.12 as rust_alpine
RUN apk update && apk add build-base git gcc g++

RUN mkdir -p /src && \
    cd /src && \
    git clone --depth 1 https://github.com/Jarusk/pass-rs.git && \
    cd pass-rs && \
    cargo build --release

FROM alpine:3.14 as tool
COPY --from=rust_alpine /src/pass-rs/target/release/pass-rs /usr/local/bin
ENTRYPOINT [ "pass-rs" ]
CMD [ "--help" ]
