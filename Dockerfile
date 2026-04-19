FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY github-recon /app/github-recon

ENV GITHUB_TOKEN=""

ENTRYPOINT ["/app/github-recon"]
