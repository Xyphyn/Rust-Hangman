# Use rust as the build container
FROM rust:alpine AS builder

# Copy the files over
WORKDIR /usr/src/app
COPY src ./src
COPY Cargo.lock Cargo.toml ./

# Build the program
RUN cargo build --release


# Use an alpine image
FROM alpine

# Copy the binary over
COPY --from=builder /usr/src/app/target/release/hangman .

# Execute it
ENTRYPOINT [ "/hangman" ]
