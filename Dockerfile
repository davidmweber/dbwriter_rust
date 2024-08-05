FROM ubuntu:latest

RUN apt-get update &&  \
    apt-get install -y libpq-dev

WORKDIR /app

COPY target/release/dbwriter_rust /app/dbwriter_rust

ENTRYPOINT ["/app/dbwriter_rust"]

EXPOSE 8080