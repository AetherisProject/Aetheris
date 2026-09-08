# Browser extension CI — zip build + manifest validation
FROM python:3.12-bookworm
RUN apt-get update && apt-get install -y zip curl
WORKDIR /aetheris/browser/chrome
COPY browser/chrome/ ./
CMD ["bash","-c","python3 -c \"import json; json.load(open('manifest.json'))\"; zip -r ../dist/aetheris-browser.zip .; echo 'Browser CI ready'"]
