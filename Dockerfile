# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory in the container
WORKDIR /chess

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project and install dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/chess*

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory in the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /chess/target/release/chess .

# Expose the port on which the application will run
EXPOSE 8080
EXPOSE 8081

# Run the Rust application
CMD ["./chess"]