FROM rust:1.90

WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
CMD ["cargo", "run"]
