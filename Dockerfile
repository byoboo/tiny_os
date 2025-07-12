# TinyOS Development Environment Dockerfile
# Multi-stage build for development and CI environments

FROM rust:1.75-bullseye as base

# Install system dependencies
RUN apt-get update && apt-get install -y \
    qemu-system-aarch64 \
    llvm \
    gcc-aarch64-linux-gnu \
    gdb-multiarch \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    vim \
    htop \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain components
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add aarch64-unknown-none && \
    rustup component add rustfmt clippy llvm-tools-preview

# Verify QEMU and tools
RUN qemu-system-aarch64 --version && \
    llvm-objcopy --version && \
    aarch64-linux-gnu-gcc --version

# Development stage
FROM base as development

# Create non-root user for development
RUN useradd -m -s /bin/bash -u 1000 dev && \
    usermod -aG sudo dev && \
    echo "dev ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Install development tools
RUN cargo install cargo-watch cargo-expand cargo-tree

# Set up workspace
WORKDIR /workspace
RUN chown -R dev:dev /workspace

# Switch to development user
USER dev

# Set environment variables
ENV CARGO_HOME=/home/dev/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

# Default command
CMD ["bash"]

# CI stage (minimal for CI/CD)
FROM base as ci

# CI-specific setup
WORKDIR /workspace

# Keep as root for CI operations
ENV CARGO_HOME=/root/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

# Default command for CI
CMD ["bash"]
