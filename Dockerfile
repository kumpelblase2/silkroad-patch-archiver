FROM rust:alpine as builder

WORKDIR /usr/src/patch-archiver
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /
COPY --from=builder /usr/src/patch-archiver/target/x86_64-unknown-linux-musl/release/silkroad-patch-archiver /silkroad-patch-archiver

VOLUME /patches
CMD ["/silkroad-patch-archiver"]
