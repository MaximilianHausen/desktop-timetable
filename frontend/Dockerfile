FROM rust as builder
ADD "https://api.github.com/repos/MaximilianHausen/desktop-dashboard/commits?per_page=1" docker_cachebust
RUN cargo install dioxus-cli
WORKDIR /usr/src
RUN git clone https://github.com/MaximilianHausen/desktop-dashboard.git
WORKDIR /usr/src/frontend
ENV CLIENT_ID="" BACKEND_URL=""
RUN dioxus build --release

FROM nginx:alpine
COPY --from=builder /usr/src/frontend/dist /usr/share/nginx/html
EXPOSE 80
