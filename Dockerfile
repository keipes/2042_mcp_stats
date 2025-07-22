# syntax=docker/dockerfile:1
FROM rust:latest AS builder

# Install cross for cross-compilation
RUN cargo install cross

WORKDIR /app
COPY . .

# Build for x86_64 Linux
RUN cross build --release --target x86_64-unknown-linux-gnu
# Build for aarch64 Linux
RUN cross build --release --target aarch64-unknown-linux-gnu
# Build for x86_64 Windows
RUN cross build --release --target x86_64-pc-windows-gnu
# Build for aarch64 Windows
RUN cross build --release --target aarch64-pc-windows-gnu

# Output binaries will be in target/<target>/release/
