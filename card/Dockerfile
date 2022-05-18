
FROM rust as builder
WORKDIR /app

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y libprotobuf-dev protobuf-compiler

COPY . .
RUN cargo build --release

FROM rust as runtime
COPY --from=builder /app/target/release/server /usr/local/bin

ENTRYPOINT ["./usr/local/bin/server"]
EXPOSE 50051