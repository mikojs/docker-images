FROM alpine

# install packages
RUN apk add \
  git \
  go \
  docker

# act
RUN git clone https://github.com/nektos/act.git
RUN act/install.sh && \
  rm -rf act

# remove not used packages
RUN apk del \
  git \
  go

# run
WORKDIR /repo
ENTRYPOINT ["act"]
