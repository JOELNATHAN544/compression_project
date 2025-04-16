# Use Rust official image as builder
FROM rust:latest as builder

# Set working directory
WORKDIR /usr/src/rust-compressor

# Copy only the dependency files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the actual source code
COPY . .

# Build the release binary
RUN cargo build --release && \
    ls -l target/release/compression

# Use a minimal image for the final container
FROM debian:bullseye-slim

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust-compressor/target/release/compression /usr/local/bin/compression

# Set the entrypoint to the CLI
ENTRYPOINT ["/usr/local/bin/compression"]
CMD ["--help"]
