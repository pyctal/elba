FROM rust:1.80

WORKDIR /data_service
COPY . .
RUN apt-get update
RUN apt-get install openssl libpq-dev gcc -y
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

# TAG elba/dev:1