# Use a Rust Docker image as the base
FROM rust:1.76-alpine3.19 as builder

# Set the working directory inside the container
WORKDIR /app

# Install necessary packages
RUN apk update && \
    apk add --no-cache musl-dev pkgconfig openssl-dev

# Copy the Rust project files to the container
COPY Cargo.toml .
COPY src/ /app/src

# Define a build argument with a default value
ARG BINARY_NAME=lrbooks

# Build the Rust project
# See: https://github.com/rust-lang/rust/issues/115430
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release --bin ${BINARY_NAME}

# Start a new stage from Alpine Linux
FROM alpine:3.19

# Install required packages
RUN apk update && \
    apk add --no-cache libgcc

# Define an environment variable from the build argument
ENV BINARY_NAME=lrbooks

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the previous stage to the current stage
COPY --from=builder /app/target/release/${BINARY_NAME} .

# Command to run the binary when the container starts
CMD ./${BINARY_NAME}
