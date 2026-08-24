# =============================================================================
# Development Container for Paladin
# Optimized for development with all tools and debugging capabilities
# Note: Rust version must match `channel` in the repo-root rust-toolchain.toml,
#       otherwise rustup will download a second toolchain on first cargo run.
# =============================================================================
FROM rust:1.97.1-slim-bullseye

# Set working directory
WORKDIR /workspace

# Avoid warnings by switching to noninteractive
ENV DEBIAN_FRONTEND=noninteractive

# Configure apt and install packages
RUN apt-get update \
    && apt-get -y install --no-install-recommends \
    # Essential build tools
    build-essential \
    pkg-config \
    libssl-dev \
    g++ \
    cmake \
    # Development tools
    git \
    curl \
    wget \
    vim \
    nano \
    jq \
    # Debugging tools
    gdb \
    lldb \
    valgrind \
    strace \
    # Database clients
    sqlite3 \
    default-mysql-client \
    redis-tools \
    # Network tools
    net-tools \
    iputils-ping \
    dnsutils \
    netcat-openbsd \
    # Process tools
    htop \
    procps \
    lsof \
    # SSL/TLS tools
    ca-certificates \
    openssl \
    # Shell enhancements
    zsh \
    bash-completion \
    # Shell linting (drives `make lint-shell` + the shellcheck pre-commit hook)
    shellcheck \
    # Documentation
    man-db \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Switch back to dialog for any ad-hoc use of apt-get
ENV DEBIAN_FRONTEND=dialog

# Install pipx + pre-commit (commit/push quality gates) system-wide so the tool
# is available to the non-root user without a per-user install.
ENV PIPX_HOME=/usr/local/pipx \
    PIPX_BIN_DIR=/usr/local/bin
RUN apt-get update \
    && apt-get install -y --no-install-recommends pipx \
    && PIPX_HOME=$PIPX_HOME PIPX_BIN_DIR=$PIPX_BIN_DIR pipx install pre-commit \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install Rust components
RUN rustup component add \
    rustfmt \
    clippy \
    rust-src \
    llvm-tools-preview

# Install essential Rust tools.
# `--locked` is mandatory: without it cargo re-resolves every transitive dep to
# the newest published version, which can pull in a crate whose MSRV exceeds the
# pinned toolchain above and break the image build (e.g. kstring 2.0.4 requiring
# rustc 1.96 while this image is on 1.95). `--locked` uses each tool's own
# published Cargo.lock, which is known-good for that tool's MSRV.
RUN cargo install --locked cargo-watch
RUN cargo install --locked cargo-edit
RUN cargo install --locked cargo-audit
RUN cargo install --locked cargo-expand
RUN cargo install --locked cargo-insta
RUN cargo install --locked cargo-public-api

# Install the doc-build toolchain (pinned) — mirrors .github/workflows/docs.yml:44-54
# exactly, so `mdbook build docs/` run in this container and in CI cannot disagree on
# tooling. Consumer: `mdbook build docs/` (no `make` target).
RUN cargo install --locked --version 0.4.40 mdbook
RUN cargo install --locked --version 0.13.0 mdbook-mermaid
RUN cargo install --locked --version 0.7.7 mdbook-linkcheck

# Install sqlx-cli for database migrations
RUN cargo install --locked sqlx-cli --no-default-features --features sqlite,mysql

# Install cargo-nextest (optional, can fail without breaking the build)
RUN cargo install --locked cargo-nextest || echo "cargo-nextest installation failed, skipping"

# Optional CLI tools - uncomment to install:
# RUN cargo install bat exa ripgrep fd-find tokei hyperfine

# Create a non-root user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Set up cargo permissions for non-root user
RUN mkdir -p /usr/local/cargo/registry \
    && chown -R $USER_UID:$USER_GID /usr/local/cargo

# Set up workspace permissions
RUN mkdir -p /workspace \
    && chown -R $USER_UID:$USER_GID /workspace

# Switch to non-root user
USER $USERNAME

# Set up shell configuration for vscode user
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc \
    && echo 'export RUST_BACKTRACE=1' >> ~/.bashrc \
    && echo 'alias ll="ls -la"' >> ~/.bashrc

# Set environment variables
ENV RUST_BACKTRACE=1
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH

# Configure git safe directory (for mounted volumes)
RUN git config --global --add safe.directory /workspace

# Set the default shell
SHELL ["/bin/bash", "-c"]

# Expose common ports
EXPOSE 8080 9090

# Default command
CMD ["/bin/bash"]
