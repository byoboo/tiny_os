# TinyOS Development Environment Dockerfile
# Multi-stage build for development and CI environments

FROM ubuntu:22.04 as base

# Install Rust
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && rm -rf /var/lib/apt/lists/*

# Add Rust to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install system dependencies with newer QEMU
RUN apt-get update && apt-get install -y \
    software-properties-common \
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

# Try to install a newer version of QEMU with raspi4b support
RUN apt-get update && apt-get install -y \
    qemu-system-aarch64 \
    && rm -rf /var/lib/apt/lists/*

# Check if we can install from backports or build from source for raspi4b support
RUN QEMU_VERSION=$(qemu-system-aarch64 --version | head -1 | cut -d' ' -f4) && \
    echo "Installed QEMU version: $QEMU_VERSION" && \
    qemu-system-aarch64 -machine help | grep -i raspi || \
    echo "Note: raspi4b not available in this QEMU version"

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

# Install development tools as root first
RUN echo "Installing development tools..." && \
    cargo install cargo-watch cargo-expand cargo-tree && \
    echo "✅ Development tools installed successfully"

# Set up workspace
WORKDIR /workspace
RUN chown -R dev:dev /workspace

# Copy rustup environment for dev user
RUN cp -r /root/.cargo /home/dev/ && \
    chown -R dev:dev /home/dev/.cargo && \
    cp -r /root/.rustup /home/dev/ && \
    chown -R dev:dev /home/dev/.rustup

# Switch to development user
USER dev

# Set environment variables for dev user
ENV CARGO_HOME=/home/dev/.cargo
ENV RUSTUP_HOME=/home/dev/.rustup
ENV PATH=$CARGO_HOME/bin:$PATH

# Default command
CMD ["bash"]

# CI stage (minimal for CI/CD)
FROM base as ci

# Install CI tools (minimal - TinyOS has no external dependencies)
RUN echo "✅ CI stage: No external Cargo tools needed (avoiding cargo-audit/cargo-outdated)" && \
    echo "Installed tools:" && \
    cargo --version && \
    rustc --version

# CI-specific setup
WORKDIR /workspace

# Keep as root for CI operations
ENV CARGO_HOME=/root/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

# Default command for CI
CMD ["bash"]
