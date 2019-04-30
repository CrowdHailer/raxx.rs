FROM rust:1.34.1

WORKDIR /usr/src/my_app
RUN rustup component add rustfmt
# RUN cargo install diesel_cli
RUN cargo install watchexec

ENV RUST_BACKTRACE=1

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN mkdir ./src
RUN touch ./src/lib.rs
RUN cargo build
RUN rm src/*.rs

COPY . .

CMD ["watchexec", "-r", "-w", "src", "cargo", "run"]
