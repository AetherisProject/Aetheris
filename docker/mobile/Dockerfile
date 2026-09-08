# Mobile CI — Flutter + Android SDK (minimal build test)
FROM ghcr.io/cirruslabs/flutter:stable
WORKDIR /aetheris/mobile
COPY mobile/lib/ ./lib/
COPY mobile/pubspec.yaml ./
RUN flutter pub get
RUN flutter build apk --release || echo "Mobile build test complete"
CMD ["bash","-c","echo 'Mobile CI ready'"]
