FROM rust:latest as build

WORKDIR /usr/src/app

COPY . .

RUN rustup component add rustfmt
RUN cd searcher-api && cargo build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /user/src/app/target/release/jito-backrun-example /usr/local/bin/
COPY --from=build /user/src/app/target/release/searcher-api /usr/local/bin/

WORKDIR /usr/local/bin

CMD ["jito-backrun-example","&","searcher-api"]


