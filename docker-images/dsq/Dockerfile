FROM golang:alpine

# install packages
RUN apk add \
  gcc \
  musl-dev

# dsq
RUN go install github.com/multiprocessio/dsq@latest

# bin
COPY ./bin/entrypoint /bin/entrypoint

# run
ENTRYPOINT ["entrypoint", "dsq"]
