FROM node:lts-alpine

# install packages
RUN apk add \
  # code-server
  curl \
  openssl \
  python3 \
  alpine-sdk \
  bash \
  libstdc++ \
  libc6-compat \
  # docker
  docker \
  docker-compose
RUN npm config set python python3
# FIXME: https://github.com/coder/code-server/issues/5530
RUN yarn global add \
  code-server@4.6.0

# code-server
RUN mkdir -p /root/.local/share/code-server/logs

# bin
COPY ./bin/entrypoint /bin/entrypoint
COPY ./bin/code /bin/code
COPY ./bin/ddocker /bin/ddocker

# run
ENTRYPOINT ["entrypoint", "code-server /root/work"]
