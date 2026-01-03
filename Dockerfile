FROM rust:1-slim-bookworm

WORKDIR /usr/epoll

COPY . .

RUN cargo build

CMD ["cargo", "run", "delayserver"]