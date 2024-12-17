## Debian (plus lourd qu'Alpine : mais au moins ça marche)
FROM rust:latest

RUN apt update
RUN apt install pkg-config -y
RUN apt install libssl-dev -y
##

## Alpine (ne fonctionne pas actuellement à cause de v8 : https://github.com/denoland/rusty_v8/issues/49 )
#FROM rust:alpine
# ENV OPENSSL_DIR=/usr
# RUN apk add --no-cache --update libc-dev pkgconfig openssl-dev python3 ## via libc-dev
# RUN apk add --no-cache --update musl-dev pkgconfig openssl-dev python3 ## via musl-dev
##

WORKDIR /app

COPY ./src ./src
# COPY ./tests ./tests
# COPY ./.env ./.env
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./tuono.config.ts ./tuono.config.ts

# binstall permet de dl seulement les bin (et donc pas le code source + compilation en bin)
RUN ["cargo", "install", "cargo-binstall"]

# cargo-llvm-cov : permet de générer des rapports de tests + code coverage
# RUN ["cargo", "binstall", "cargo-llvm-cov" , "--secure" , "--no-confirm"]

# cargo-nextest : improve le `cargo test` natif en améliorant entre autres le temps d'exe des tests
# RUN ["cargo", "binstall", "cargo-nextest" , "--secure" , "--no-confirm"]

# cargo-mutants : permet d'exe des tests par mutation
# RUN ["cargo", "binstall", "cargo-mutants" , "--secure" , "--no-confirm"]

RUN ["cargo", "binstall", "tuono"]
# RUN ["tuono", "dev"]  # commenté pour debug (provoque une erreur)

# ENTRYPOINT tail -f /dev/null
# ENTRYPOINT ["./target/debug/tuono"] # commenté pour debug