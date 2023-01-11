FROM rust as builder
ADD "https://api.github.com/repos/MaximilianHausen/desktop-dashboard/commits?per_page=1" docker_cachebust
RUN git clone https://github.com/MaximilianHausen/desktop-dashboard.git /usr/src
WORKDIR /usr/src
RUN cargo build -p backend --release

FROM debian:stable-slim
COPY --from=builder /usr/src/target/release/backend /usr/local/bin
EXPOSE 3000
CMD ["/usr/local/bin/backend"]
