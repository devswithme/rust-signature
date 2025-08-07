FROM rust:1-alpine3.20 as builder
RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release
FROM alpine:3.20

COPY --from=builder /app/target/release/signature /

CMD ["./signature"]