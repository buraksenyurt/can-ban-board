FROM rust:1.78.0-bullseye as build
WORKDIR /usr/src/shared
COPY shared .
WORKDIR /usr/src/wi_api
COPY backend .
RUN cargo build --release
FROM debian:buster-slim
ARG APP=/usr/src/app
EXPOSE 4448
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata sqlite3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/wi_api/target/release/wi_api ${APP}/wi_api
COPY --from=build /usr/src/wi_api/can_ban.db ${APP}/can_ban.db
WORKDIR ${APP}
CMD ["./wi_api"]