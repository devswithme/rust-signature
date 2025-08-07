FROM rust:1-alpine3.20 as builder

RUN apk add --no-cache musl-dev build-base

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/signature .

EXPOSE 3000

CMD ["./signature"]