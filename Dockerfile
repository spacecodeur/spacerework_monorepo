## Debian (plus lourd qu'Alpine : mais au moins ça marche)
FROM rust:bookworm

RUN ["apt", "update"]

# Packages pour tuono
RUN ["apt", "install", "pkg-config", "-y"]
RUN ["apt", "install", "libssl-dev", "-y"]

# Installation de nodejs
RUN ["apt", "install", "nodejs", "-y"]
RUN ["apt", "install", "npm", "-y"]

WORKDIR /app

COPY . /app/

# # binstall permet de dl seulement les bin (et donc pas le code source + compilation en bin)
# RUN ["cargo", "install", "cargo-binstall"]
# # cargo-llvm-cov : permet de générer des rapports de tests + code coverage
# RUN ["cargo", "install", "cargo-llvm-cov" , "--secure" , "--no-confirm"]
# # cargo-nextest : improve le `cargo test` natif en améliorant entre autres le temps d'exe des tests
# RUN ["cargo", "install", "cargo-nextest" , "--secure" , "--no-confirm"]
# # cargo-mutants : permet d'exe des tests par mutation
# RUN ["cargo", "install", "cargo-mutants" , "--secure" , "--no-confirm"]

# inutile de passer par binstall pour tuono : le gain en temps est négligeable
RUN ["cargo", "install", "tuono"]

RUN ["npm", "install"]

# ENTRYPOINT tail -f /dev/null
# ENTRYPOINT ["./target/debug/tuono"] # commenté pour debug