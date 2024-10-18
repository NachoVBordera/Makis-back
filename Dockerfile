# Use the official Rust image as a builder
FROM rust:1.72 as builder

# Create a new directory for the application
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to generate a lockfile
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release

# Now copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Production stage using distroless
FROM gcr.io/distroless/cc

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/makis .

ENV DATABASE_URL=postgres://postgres:postgres@db:5432/postgres

# Run the binary
CMD ["./makis"]
