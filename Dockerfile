FROM rust as builder

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN rustup target install x86_64-unknown-linux-musl

WORKDIR /src/code

COPY . .

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder /src/code/target/x86_64-unknown-linux-musl/release/wasm /bin/wasm

ENTRYPOINT ["/bin/wasm"]