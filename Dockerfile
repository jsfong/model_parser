# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN apk add --no-cache \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    musl-dev \
    gcc

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
# COPY . .
# Copy source codes and toml
COPY .sqlx ./.sqlx
COPY assets ./assets
COPY end2end ./end2end
COPY src ./src
COPY style ./style
COPY Cargo.toml Cargo.lock ./

# Install sqlx-cli
RUN cargo install sqlx-cli

# Enable offline mode for sqlx
ENV SQLX_OFFLINE=true

# Save queries
RUN cargo sqlx prepare --check

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/leptos_model_parser /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/leptos_model_parser"]
