# Web CI — Node + Vite build/test
FROM node:20-bookworm
RUN apt-get update && apt-get install -y curl python3
WORKDIR /aetheris/web
COPY web/package*.json ./
RUN npm ci
COPY web/ ./
CMD ["bash","-c","npm run build && echo 'Web build OK'"]
