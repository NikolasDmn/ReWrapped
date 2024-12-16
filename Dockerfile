FROM rust:latest as builder

WORKDIR /app

# Install trunk (for Yew) and wasm-pack
RUN cargo install trunk wasm-pack
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y npm
RUN npm i -D tailwindcss daisyui@latest
COPY . .

RUN trunk build --release

FROM nginx:alpine

WORKDIR /usr/share/nginx/html

COPY --from=builder /app/dist .

EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
