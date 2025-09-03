# Build stage
FROM rust:1.88-alpine3.21 AS builder

RUN apk add --no-cache \
    musl-dev \ 
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage with Ubuntu questing
FROM ubuntu:questing

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/lgr_ehr /app/lgr_ehr

# Create logs directory
RUN mkdir -p /app/logs

# Expose app port
EXPOSE 3000

# Run the binary
CMD ["/app/lgr_ehr"]