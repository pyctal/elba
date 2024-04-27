FROM rust:1.77

WORKDIR /data_service
COPY . . 
RUN apt-get upgrade
RUN apt-get install openssl libpq-dev gcc
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel migration generate --diff-schema create_match_data
RUN diesel migration run
RUN cargo install cargo-watch

# TAG elba/dev:1