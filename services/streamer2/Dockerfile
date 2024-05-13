FROM rust:slim-bullseye as build

RUN USER=root cargo new --bin streamer2
WORKDIR /streamer2

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/streamer2*

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=build /streamer2/target/release/streamer2 .

CMD ["./streamer2"]
