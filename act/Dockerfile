FROM alpine

RUN apk add \
  git \
  go \
  docker

# act
RUN git clone https://github.com/nektos/act.git
RUN act/install.sh && \
  rm -rf act

# Remove not used
RUN apk del \
  git \
  go

# run act
ENTRYPOINT ["act"]
