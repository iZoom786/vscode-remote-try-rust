# Use the official Rust image as the base image
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:bullseye-slim

# Install necessary libraries
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/hello_remote_world .

# Expose the port the app runs on
EXPOSE 8080

# Command to run the application
CMD ["./hello_remote_world"]