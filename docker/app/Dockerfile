# Use the specified Rust version and base OS as the base image
ARG BASE_OS
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-${BASE_OS}

# Nodejs installation

RUN ["mkdir", "/usr/local/nvm"]
WORKDIR /usr/local/node

ARG NODE_VERSION

## Download the Node.js standelone binary archive && extract it && clean files

RUN ["/bin/sh", "-c", "wget https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz"]
RUN ["/bin/sh", "-c", "tar xf node-v${NODE_VERSION}-linux-x64.tar.xz --strip-components 1"]
RUN ["/bin/sh", "-c", "rm node-v${NODE_VERSION}-linux-x64.tar.xz"]

# Ensure Node.js binaries are available in the PATH

ENV PATH /usr/local/node/bin:$PATH

# binstall allows downloading only binaries (avoiding time spent downloading and compiling from source)
RUN ["cargo", "install", "cargo-binstall"]
RUN ["cargo", "binstall", "diesel_cli", "--secure", "--no-confirm"]
RUN ["cargo", "binstall", "wasm-pack", "--secure", "--no-confirm"]

# cargo-llvm-cov: generates test reports and code coverage
RUN ["cargo", "binstall", "cargo-llvm-cov", "--secure", "--no-confirm"]
# cargo-nextest: improves the native `cargo test` by speeding up test execution
RUN ["cargo", "binstall", "cargo-nextest", "--secure", "--no-confirm"]
# cargo-mutants: allows running mutation tests
RUN ["cargo", "binstall", "cargo-mutants", "--secure", "--no-confirm"]

# global configuration done ! let's focus on the project itself

# Set the working directory to /app
WORKDIR /app

# Copy project files into the container (excepted files/dir declared in .dockerignore)
COPY . .

RUN ["npm", "install"]

ARG TUONO_VERSION
RUN ["/bin/sh", "-c", "cargo install tuono@^${TUONO_VERSION}"]