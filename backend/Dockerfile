FROM rust:slim as builder
COPY Cargo.toml /usr/src/backend/
COPY src /usr/src/backend/src/
WORKDIR /usr/src/backend
RUN cargo build -p backend --release

FROM debian:stable-slim
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin
ENV CLIENT_ID="" CLIENT_SECRET=""
EXPOSE 3000
CMD ["/usr/local/bin/backend"]
