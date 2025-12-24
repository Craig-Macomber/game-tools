FROM mcr.microsoft.com/devcontainers/rust:latest

# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install git

# https://www.cybertec-postgresql.com/en/dev-container-for-pgrx-postgresql-extensions-lessons-learned/
USER vscode

# https://github.com/EmbarkStudios/cargo-about?tab=readme-ov-file#from-cratesio
RUN cargo install --locked cargo-about

# https://dioxuslabs.com/learn/0.7/getting_started/
RUN cargo install dioxus-cli
