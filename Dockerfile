FROM rust as builder
ENV LEPTOS_ENV="PROD"
ADD "https://api.github.com/repos/MaximilianHausen/desktop-timetable/commits?per_page=1" docker_cachebust
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
RUN git clone https://github.com/MaximilianHausen/desktop-timetable.git /usr/src
WORKDIR /usr/src
RUN cargo leptos build -r

FROM debian:stable-slim
ENV HW_CLIENT_ID="CLIENT_ID" HW_SECRET="SECRETSECRETSECRETSECRETSECRETSECRETSECRETSECRETSECRETSECRETSECRET"
ENV LEPTOS_OUTPUT_NAME="desktop_timetable" LEPTOS_SITE_ADDR="0.0.0.0:80"
COPY --from=builder /usr/src/Cargo.toml /usr/bin/desktop_timetable/Cargo.toml
COPY --from=builder /usr/src/target/server/release/desktop_timetable /usr/bin/desktop_timetable/desktop_timetable
COPY --from=builder /usr/src/target/site /usr/bin/desktop_timetable/target/site
WORKDIR /usr/bin/desktop_timetable
ENTRYPOINT ["./desktop_timetable"]
EXPOSE 80
