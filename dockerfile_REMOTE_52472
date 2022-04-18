FROM rust:1.60.0

WORKDIR /api
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/project-api"]