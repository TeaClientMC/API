# Use the official Rust image as a parent image
FROM rust:1.74.0 

# Set the working directory in the Docker image
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

# Expose port 8000
EXPOSE 8080:8080

# Set the command to run when the Docker container starts
CMD ["cargo", "run", "--release"]