FROM rust:1.90

WORKDIR /app

RUN cargo install cargo-watch

RUN cargo install sqlx-cli --no-default-features --features=native-tls,postgres

COPY . .

CMD ["cargo", "watch", "-x", "run"]


# 以下本番用構成なので一旦コメントアウト

# FROM rust:1.90 as builder

# WORKDIR /app

# COPY Cargo.toml Cargo.lock ./

# RUN mkdir src && \
#     echo "fn main() { println!(\"if you see this, the build broken.\")}" > src/main.rs && \
#     cargo build --release && \
#     rm -rf target/release/deps/ghost_api*

# COPY . .

# RUN cargo build --release

# FROM debian:bookworm-slim

# WORKDIR /app

# RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /app/target/release/ghost_api /app/ghost_api

# EXPOSE 8080

# CMD ["./ghost_api"]

