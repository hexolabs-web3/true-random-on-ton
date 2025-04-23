FROM rustlang/rust:nightly

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo +nightly build --release

COPY ./src ./src
RUN cargo +nightly build --release

CMD ["cargo", "run"]
