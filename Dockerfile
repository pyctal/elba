FROM rust:1.67

WORKDIR /elba
COPY . . 

ENTRYPOINT ["tail", "-f", "/dev/null"]

# TAG elba/dev:1