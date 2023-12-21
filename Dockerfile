FROM rust:slim-bookworm as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /opt/resource
COPY --from=build /build/target/release/concourse-github-issue main
RUN ln -s main check && ln -s main in && ln -s main out
