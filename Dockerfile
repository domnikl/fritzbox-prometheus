FROM rust:1.67-alpine AS builder
WORKDIR /app
COPY . .
RUN apk add libc-dev && rustup component add clippy \
    && cargo clippy --no-deps \
    && cargo check \
    && cargo build

FROM alpine
WORKDIR /app
EXPOSE 9000
COPY --from=builder /app/target/debug/fritzbox-prometheus .
CMD ["./fritzbox-prometheus"]
