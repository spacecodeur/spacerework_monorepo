# Use the specified Rust version and base OS as the base image
ARG BASE_OS
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-${BASE_OS}

RUN ["apt", "update"]

# Nodejs installation
RUN ["mkdir", "/usr/local/nvm"]
ENV NVM_DIR /usr/local/nvm

RUN ["apt", "update"]
RUN ["apt", "install", "curl", "-y", "--no-install-recommends"]

RUN ["curl", "https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh", "-o", "/tmp/install_nvm.sh"]
RUN ["bash", "/tmp/install_nvm.sh"]

ARG NODE_VERSION
RUN ["sh", "-c", ". /usr/local/nvm/nvm.sh && nvm install ${NODE_VERSION}"]

# Ensure Node.js binaries are available in the PATH
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# Install bin & packages

## packages Needed to install Tuono
RUN ["apt", "install", "pkg-config", "-y"] 
RUN ["apt", "install", "libssl-dev", "-y"]
RUN ["cargo", "install", "tuono"]

# # binstall allows downloading only binaries (avoiding time spent downloading and compiling from source)
# RUN ["cargo", "install", "cargo-binstall"]
# # cargo-llvm-cov: generates test reports and code coverage
# RUN ["cargo", "binstall", "cargo-llvm-cov", "--secure", "--no-confirm"]
# # cargo-nextest: improves the native `cargo test` by speeding up test execution
# RUN ["cargo", "binstall", "cargo-nextest", "--secure", "--no-confirm"]
# # cargo-mutants: allows running mutation tests
# RUN ["cargo", "binstall", "cargo-mutants", "--secure", "--no-confirm"

# Set the working directory to /app
WORKDIR /app

# Copy project files into the container (excepted files/dir declared in .dockerignore)
COPY . .
