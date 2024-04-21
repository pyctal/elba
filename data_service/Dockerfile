FROM rust:1.77-alpine

WORKDIR /data_service
COPY . . 
RUN apk add musl-dev pkgconfig libressl-dev
RUN cargo install cargo-watch

# TAG elba/dev:1