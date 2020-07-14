FROM rustlang/rust:nightly

WORKDIR /patchup/server

# cache cargo crates
COPY Cargo.toml ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build
RUN rm -f target/release/deps/server*

# build rust server
COPY . ./
RUN cargo build

# setup cargo watch
RUN cargo install cargo-watch

EXPOSE 8000
CMD ["cargo", "watch", "-x", "run"]
