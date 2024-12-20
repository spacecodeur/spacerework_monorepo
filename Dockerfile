## Using Debian instead of Alpine because V8 compilation fails due to a tricky bug: https://github.com/denoland/rusty_v8/issues/49
FROM rust:bookworm

RUN ["apt", "update"]

# Packages needed to later install Tuono
RUN ["apt", "install", "pkg-config", "-y"]
RUN ["apt", "install", "libssl-dev", "-y"]

# Installing Node.js
RUN ["apt", "install", "nodejs", "-y"]
RUN ["apt", "install", "npm", "-y"]

WORKDIR /app

# Remember to use a .dockerignore file to exclude unwanted files/directories
COPY . /app/

# # binstall allows downloading only binaries (avoiding time spent downloading and compiling from source)
# RUN ["cargo", "install", "cargo-binstall"]
# # cargo-llvm-cov: generates test reports and code coverage
# RUN ["cargo", "binstall", "cargo-llvm-cov", "--secure", "--no-confirm"]
# # cargo-nextest: improves the native `cargo test` by speeding up test execution
# RUN ["cargo", "binstall", "cargo-nextest", "--secure", "--no-confirm"]
# # cargo-mutants: allows running mutation tests
# RUN ["cargo", "binstall", "cargo-mutants", "--secure", "--no-confirm"]

# No need to use binstall for Tuono: the time saved is negligible
RUN ["cargo", "install", "tuono"]

RUN ["npm", "install"]
