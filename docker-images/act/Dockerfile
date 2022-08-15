FROM golang:alpine as build

# install packages
RUN apk add \
  git

# act
RUN git clone https://github.com/nektos/act.git
RUN act/install.sh

FROM alpine

COPY --from=build /go/bin/act /bin/act

# install packages
RUN apk add \
  docker

# run
ENTRYPOINT ["act"]
