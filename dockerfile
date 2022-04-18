<<<<<<< HEAD
FROM rust:1.60

WORKDIR /api

COPY . .

EXPOSE 8080

RUN cargo build --release

CMD ["./target/build/project-api"]
=======
FROM rust:1.60.0

WORKDIR /api
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
EXPOSE 8080

ENTRYPOINT ["./target/release/project-api"]
>>>>>>> newdb
