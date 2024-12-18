FROM node:20-bookworm as nodemodules-builder

WORKDIR /app
COPY ./package.json /app/package.json
RUN npm install

## Debian (plus lourd qu'Alpine : mais au moins ça marche)
FROM rust:bookworm as tuono-builder

RUN apt update
RUN apt install pkg-config -y
RUN apt install libssl-dev -y
##

WORKDIR /app

RUN ["cargo", "install", "cargo-binstall"]
RUN ["cargo", "binstall", "tuono"]

FROM debian:bookworm

WORKDIR /app

COPY --from=tuono-builder /usr/local/cargo/bin/tuono /usr/local/bin/tuono
COPY --from=nodemodules-builder /app/node_modules /app/node_modules
COPY . /app/
RUN apt install libssl-dev -y
# RUN ["tuono", "dev"]  # démarre les serveur front et back de dev, je commente pour le moment car cette ligne provoque une erreur (sans doute parce que nodejs n'est pas installé)commenté pour debug (provoque une erreur)

# ENTRYPOINT tail -f /dev/null
# ENTRYPOINT ["./target/debug/tuono"] # commenté pour debug