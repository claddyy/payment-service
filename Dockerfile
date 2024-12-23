FROM rust:1.83

WORKDIR /app

COPY . .

# Rename .env.example to .env & set the correct database URL
RUN mv .env.example .env && \
    sed -i 's|DATABASE_URL=.*|DATABASE_URL=postgres://postgres:postgres@db:5432/server|g' .env

RUN cargo build --release

# Install sea-orm-cli for migrations
RUN cargo install sea-orm-cli@1.1.0

# Create a startup script
RUN echo '#!/bin/bash\n\
cd db && sea-orm-cli migrate fresh && cd .. && \
./target/release/http' > /app/start.sh && \
chmod +x /app/start.sh

EXPOSE 3000

# Run startup script that handles migrations and starts the app
CMD ["/app/start.sh"]