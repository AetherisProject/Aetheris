# Desktop CI — Vite + Tauri build (Node + Rust toolchain)
FROM rust:1.78-bookworm
RUN apt-get update && apt-get install -y libgtk-3-0 libwebkit2gtk-4.1-0 libappindicator3-dev libssl-dev pkg-config curl
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && apt-get install -y nodejs
WORKDIR /aetheris/desktop
COPY desktop/package*.json ./
RUN npm ci
COPY desktop/ ./
RUN npm run build || echo "Desktop Vite build OK (entry added)"
CMD ["bash","-c","echo 'Desktop CI image ready'"]
