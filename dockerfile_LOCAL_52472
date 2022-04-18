FROM rust:1.60

WORKDIR /api

COPY . .

EXPOSE 8080

RUN cargo build --release

CMD ["./target/build/project-api"]