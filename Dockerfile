# Build stage
FROM rust:1.88-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    ca-certificates

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy the correct binary name (should match your Cargo.toml name)
COPY --from=builder /app/target/release/lgr_ehr /app/lgr_ehr

# Copy CA certificates from builder (Alpine has them)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

EXPOSE 3000
ENTRYPOINT ["/app/lgr_ehr"]