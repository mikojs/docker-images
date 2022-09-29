FROM rust:alpine

# https://github.com/rust-lang/docker-rust/issues/68
# install packages
RUN apk add \
  musl-dev

# bin
COPY ./bin/entrypoint /bin/entrypoint

# run
ENTRYPOINT ["entrypoint", "rust"]
