FROM debian:bullseye-slim as builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc as runtime

COPY --from=builder /app/target/release/zero2prod zero2prod

ENTRYPOINT ["./zero2prod"]