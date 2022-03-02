FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/warp_server_deploy /warp_server_deploy

CMD ["/warp_server_deploy"]