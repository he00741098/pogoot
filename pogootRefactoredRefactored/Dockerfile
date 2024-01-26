# Rust as the base image
FROM rust:1.75.0
# create a new empty shell project
RUN apt-get update && apt-get upgrade -y
RUN apt-get install libssl-dev

RUN apt-get install -y -q build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /command-agent
COPY ./src/. /command-agent/src/
# COPY .env /command-agent/
COPY Cargo.toml /command-agent/
COPY Cargo.lock /command-agent/
COPY ./src/DBSecrets.toml /command-agent/

RUN cargo build --release

EXPOSE 3000
ENTRYPOINT /command-agent/target/release/pogootRefactoredRefactored
