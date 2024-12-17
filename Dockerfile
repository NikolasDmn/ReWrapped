FROM rust:latest as builder
WORKDIR /app
# Install trunk (for Yew) and wasm-pack
RUN cargo install trunk wasm-pack
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y npm
RUN npm i -D tailwindcss daisyui@latest
COPY . .
RUN trunk build --release --public-url /

FROM nginx:alpine
WORKDIR /usr/share/nginx/html
COPY --from=builder /app/dist .
COPY --from=builder /app/dist/index.html /usr/share/nginx/html/index.html
RUN echo 'server { \
    listen 80; \
    server_name localhost; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
        try_files $uri /index.html; \
    } \
}' > /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
