FROM rust:1.86-alpine

WORKDIR ../src
COPY . .

RUN cargo install --path .

ENV DATABASE_URL=$DATABASE_URL

CMD ["cargo run"]