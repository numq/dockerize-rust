FROM rust:latest
ARG SERVICE_NAME
ENV SERVICE_NAME=$SERVICE_NAME
WORKDIR /build
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
ARG SERVICE_NAME
ENV SERVICE_NAME=$SERVICE_NAME
COPY --from=0 /build/.env .
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/$SERVICE_NAME .
CMD ["/bin/sh", "-c", "./${SERVICE_NAME}"]