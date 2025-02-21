FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the migrations
COPY migrations ./migrations

# The binary will be copied during the build process
CMD ["./simple-crud-rust"] 