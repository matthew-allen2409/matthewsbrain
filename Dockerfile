FROM rust:latest

WORKDIR /app
COPY . .

EXPOSE 8080

RUN cargo build --release

CMD ["./target/release/blog-controller"]
