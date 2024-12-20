## debian et non alpine parce que la compilation v8 ne passe pas à cause d'un bug un peu velu : https://github.com/denoland/rusty_v8/issues/49
FROM rust:bookworm

RUN ["apt", "update"]

# Packages pouvoir installer ensuite tuono
RUN ["apt", "install", "pkg-config", "-y"]
RUN ["apt", "install", "libssl-dev", "-y"]

# Installation de nodejs
RUN ["apt", "install", "nodejs", "-y"]
RUN ["apt", "install", "npm", "-y"]

WORKDIR /app

# Pensez à utiliser un .dockerignore pour exclure des fichiers/dossiers non voulus 
COPY . /app/

# # binstall permet de télécharger seulement les binaires (et donc éviter de perdre du temps à télécharger puis compiler en pas le code source + compilation en bin)
# RUN ["cargo", "install", "cargo-binstall"]
# # cargo-llvm-cov : permet de générer des rapports de tests + code coverage
# RUN ["cargo", "binstall", "cargo-llvm-cov" , "--secure" , "--no-confirm"]
# # cargo-nextest : améliore le `cargo test` natif en accélérant entre autres le temps d'execution des tests
# RUN ["cargo", "binstall", "cargo-nextest" , "--secure" , "--no-confirm"]
# # cargo-mutants : permet d'exécuter des tests par mutation
# RUN ["cargo", "binstall", "cargo-mutants" , "--secure" , "--no-confirm"]

# Inutile de passer par binstall pour tuono : le gain en temps est négligeable
RUN ["cargo", "install", "tuono"]

RUN ["npm", "install"]