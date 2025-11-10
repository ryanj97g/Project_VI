# Stage 1: Build
FROM rust:1.90 as builder

WORKDIR /app

# Install build dependencies for GUI libraries (egui/eframe)
RUN apt-get update && apt-get install -y \
    libgtk-3-dev \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libasound2-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source
COPY src ./src
COPY tests ./tests
COPY examples ./examples

# Build release binary
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Ollama
RUN curl -fsSL https://ollama.com/install.sh | sh

WORKDIR /app

# Copy binaries from builder
COPY --from=builder /app/target/release/vi3 /usr/local/bin/project-vi
COPY --from=builder /app/target/release/migrate_memory /usr/local/bin/

# Copy documentation
COPY README.md LICENSE DOCUMENTATION.md COMPUTATIONAL_PHYSICS.md ./

# Create directories for runtime data
RUN mkdir -p /app/data /app/memory_archive /app/conversation_logs

# Expose Ollama port (if needed)
EXPOSE 11434

# Set environment variables
ENV RUST_LOG=info
ENV OLLAMA_BASE_URL=http://localhost:11434

# Note: This image requires Ollama models to be pulled
# Run: ollama pull gemma2:2b && ollama pull tinyllama:latest

CMD ["project-vi"]

