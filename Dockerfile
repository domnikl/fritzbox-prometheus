FROM rust:1.67-alpine
WORKDIR /app
COPY . .
RUN apk add libc-dev && cargo build
EXPOSE 9000
CMD ["target/debug/fritzbox-prometheus"]