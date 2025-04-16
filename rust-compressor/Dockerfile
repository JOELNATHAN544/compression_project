# Use Rust official image as builder
FROM rust:latest as builder

# Set working directory
WORKDIR /usr/src/rust-compressor

# Copy the source code
COPY . .

# Build the release binary
RUN cargo build --release

# Use a minimal image for the final container
FROM debian:bullseye-slim

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust-compressor/target/release/compression /usr/local/bin/compression

# Set the entrypoint to the CLI
ENTRYPOINT ["/usr/local/bin/compression"]
CMD ["--help"]
