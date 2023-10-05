# Use the minimal Alpine Linux as a base image with Rust installed
FROM rust:1.72.1-alpine

# Set the current working directory inside the container
WORKDIR /usr/src/fibonacci

# Copy the local files to the working directory
COPY . .

# Build the project (this will create the executable in the target/release directory)
RUN cargo build --release

# Run the compiled executable
CMD ["./target/release/docker_user"]
