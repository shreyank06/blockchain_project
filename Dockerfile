# Use the official Rust image as the base image for building and running
FROM rust:1.90

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the project files to the container
COPY . .

# List the files in the current directory for debugging purposes
RUN echo "Project files in the context:" && ls -la /usr/src/app


# Install necessary dependencies (SQLite, OpenSSL)
RUN apt-get update && apt-get install -y sqlite3 libssl-dev

# Build the project (this will create the compiled binary)
RUN cargo build --release

# Copy the SQLite database file (if needed)
#COPY opportunities.db .
