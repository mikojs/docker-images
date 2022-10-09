FROM rust:alpine

# https://github.com/rust-lang/docker-rust/issues/68
# install packages
RUN apk add \
  musl-dev
RUN rustup component add \
  rustfmt

# bin
COPY ./bin/entrypoint /bin/entrypoint

# run
ENTRYPOINT ["entrypoint", "rustc"]
