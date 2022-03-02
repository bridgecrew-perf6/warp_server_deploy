FROM rust as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM rust:latest-alpine

COPY --from=builder /target/release/warp_server_deploy /warp_server_deploy

CMD ["/warp_server_deploy"]