FROM rust:1.70-alpine AS builder
WORKDIR /app
COPY . .
RUN apk add libc-dev && rustup component add clippy \
    && cargo clippy --no-deps \
    && cargo check \
    && cargo build -r

FROM alpine
WORKDIR /app
EXPOSE 9000
COPY --from=builder /app/target/release/fritzbox-prometheus .
CMD ["./fritzbox-prometheus"]
