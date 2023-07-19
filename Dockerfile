FROM rust:latest
WORKDIR /build
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=0 build/.env .
COPY --from=0 build/target/x86_64-unknown-linux-musl/release/dockerize_rust .
CMD ["./dockerize_rust"]