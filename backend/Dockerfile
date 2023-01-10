FROM rust as builder
ADD "https://api.github.com/repos/MaximilianHausen/desktop-dashboard/commits?per_page=1" docker_cachebust
WORKDIR /usr/src
RUN git clone https://github.com/MaximilianHausen/desktop-dashboard.git .
RUN cargo build -p backend --release

FROM debian:stable-slim
COPY --from=builder /usr/src/backend/target/release/backend /usr/local/bin
EXPOSE 3000
CMD ["/usr/local/bin/backend"]
