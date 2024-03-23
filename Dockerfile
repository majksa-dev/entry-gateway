# Build app
FROM rust:1-slim-bullseye as builder
RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev build-essential cmake && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /app
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
RUN cp ./target/release/server ./server

# Application
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y wget libssl-dev && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /app
WORKDIR /app
ENV RUST_LOG=info
ENV PORT=80
ENV HEALTH_CHECK_PORT=9000
EXPOSE 80
COPY --from=builder /app/server /app/server
CMD [ "./server"]
HEALTHCHECK --interval=5s --timeout=5s --start-period=5s --retries=5 CMD [ "wget", "-q", "-O", "-", "http://localhost:$$HEALTH_CHECK_PORT" ]