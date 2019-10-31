FROM ekidd/rust-musl-builder:nightly-2019-11-06 as builder

COPY ./Cargo.toml ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build --release

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/server*

COPY . .

RUN cargo build --release

RUN strip ./target/x86_64-unknown-linux-musl/release/server

FROM scratch

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/server ./server
COPY ./Rocket.toml ./

CMD ["./server"]
