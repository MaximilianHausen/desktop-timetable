FROM rust as builder
ENV CLIENT_ID="CLIENT_ID" BACKEND_URL="BACKEND_URL"
ADD "https://api.github.com/repos/MaximilianHausen/desktop-dashboard/commits?per_page=1" docker_cachebust
RUN git clone https://github.com/MaximilianHausen/desktop-dashboard.git /usr/src
WORKDIR /usr/src/frontend
RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli
RUN dioxus build --release

FROM nginx:alpine
COPY --from=builder /usr/src/frontend/dist /usr/share/nginx/html
COPY --from=builder /usr/src/frontend/nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
