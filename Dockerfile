FROM rust:latest

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

WORKDIR /app
COPY . /app/
COPY .env.docker /app/.env

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Add wait-for-it script
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh /usr/wait-for-it.sh
RUN chmod +x /usr/wait-for-it.sh

# Build the Rust application
RUN cargo build --release

# Set up and run migrations when the container starts
CMD /usr/wait-for-it.sh postgres:5432 --timeout=60 && \
    diesel setup && \
    diesel migration run && \
    ./target/release/my_rust_app

