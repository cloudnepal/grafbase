# Build
FROM rust:1.73-alpine3.18 AS build

WORKDIR /grafbase

COPY ./cli ./cli
COPY ./engine ./engine

WORKDIR /grafbase/cli

RUN apk add --no-cache git musl-dev

RUN cargo build --release

# Run
FROM alpine:3.18

WORKDIR /grafbase

COPY --from=build /grafbase/cli/target/release/grafbase /bin/grafbase

ENTRYPOINT ["/bin/grafbase"]

CMD ["start"]

EXPOSE 4000
