FROM golang:alpine as build

# install packages
RUN apk add \
  git

# act
RUN git clone https://github.com/nektos/act.git
RUN act/install.sh

# main image
FROM alpine

# act
COPY --from=build /go/bin/act /bin/act

# install packages
RUN apk add \
  docker

# bin
COPY ./bin/entrypoint /bin/entrypoint

# run
ENTRYPOINT ["entrypoint", "act"]
