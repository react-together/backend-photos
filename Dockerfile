# Build
FROM rust:alpine AS builder

WORKDIR /app

RUN apk update && apk add --no-cache build-base musl-dev openssl-dev openssl-libs-static pkgconfig

COPY . .

RUN cargo build --release

# Prod
FROM alpine:latest

COPY --from=builder /app/target/release/backend-photos /app/backend-photos

EXPOSE 8000

CMD ["/app/backend-photos"]
